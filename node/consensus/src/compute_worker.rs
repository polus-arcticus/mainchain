use std::{
	pin::Pin,
	sync::{
		atomic::{AtomicUsize, Ordering},
		Arc,
	},
	time::{Duration, Instant},
};

use codec::Codec;
use futures::{
	executor::block_on,
	future::BoxFuture,
	prelude::*,
	task::{Context, Poll},
};
use futures_timer::Delay;
use log::*;
use parking_lot::Mutex;
use sc_client_api::{AuxStore, BlockchainEvents, ImportNotifications};
use sc_consensus::BoxBlockImport;
use sc_service::TaskManager;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_consensus::{Environment, Proposal, Proposer, SelectChain, SyncOracle};
use sp_core::{traits::SpawnEssentialNamed, RuntimeDebug, U256};
use sp_runtime::traits::{Block as BlockT, Header as HeaderT, NumberFor};
use sp_timestamp::Timestamp;
use ulx_bitcoin_utxo_tracker::UtxoTracker;

use ulx_node_runtime::{NotaryRecordT, NotebookVerifyError};
use ulx_primitives::{inherents::BlockSealInherentNodeSide, tick::Tick, *};

use crate::{
	aux_client::UlxAux, block_creator, block_creator::propose, compute_solver::ComputeSolver,
	digests::get_tick_digest, error::Error, notebook_watch::has_votes_at_tick,
};

/// Version of the mining worker.
#[derive(Eq, PartialEq, Clone, Copy, RuntimeDebug)]
pub struct Version(pub usize);

/// Mining metadata. This is the information needed to start an actual mining loop.
#[derive(Clone, Eq, PartialEq)]
pub struct MiningMetadata<H> {
	/// Currently known best hash which the pre-hash is built on.
	pub best_hash: H,
	pub import_time: Instant,
	pub has_tax_votes: bool,
	pub parent_tick: Tick,
}

struct MiningBuild<B: BlockT, Proof> {
	pub proposal: Proposal<B, Proof>,
	/// Mining pre-hash.
	pub pre_hash: B::Hash,
	/// Pre-runtime digest item.
	pub difficulty: ComputeDifficulty,
}

/// Mining worker that exposes structs to query the current mining build and submit mined blocks.
pub struct MiningHandle<Block: BlockT, L: sc_consensus::JustificationSyncLink<Block>, Proof> {
	version: Arc<AtomicUsize>,
	justification_sync_link: Arc<L>,
	metadata: Arc<Mutex<Option<MiningMetadata<Block::Hash>>>>,
	build: Arc<Mutex<Option<MiningBuild<Block, Proof>>>>,
	block_import: Arc<Mutex<BoxBlockImport<Block>>>,
}

impl<B, L, Proof> MiningHandle<B, L, Proof>
where
	B: BlockT,
	L: sc_consensus::JustificationSyncLink<B>,
	Proof: Send,
{
	fn increment_version(&self) {
		self.version.fetch_add(1, Ordering::SeqCst);
	}

	pub fn new(block_import: BoxBlockImport<B>, justification_sync_link: L) -> Self {
		Self {
			version: Arc::new(AtomicUsize::new(0)),
			justification_sync_link: Arc::new(justification_sync_link),
			build: Arc::new(Mutex::new(None)),
			block_import: Arc::new(Mutex::new(block_import)),
			metadata: Arc::new(Mutex::new(None)),
		}
	}

	pub(crate) fn stop_solving_current(&self) {
		let mut build = self.build.lock();
		*build = None;
		let mut metadata = self.metadata.lock();
		*metadata = None;
		self.increment_version();
	}

	pub(crate) fn on_block(&self, best_hash: B::Hash, has_tax_votes: bool, parent_tick: Tick) {
		self.stop_solving_current();
		let mut metadata = self.metadata.lock();
		*metadata = Some(MiningMetadata {
			best_hash,
			has_tax_votes,
			import_time: Instant::now(),
			parent_tick,
		});
	}

	pub(crate) fn start_solving(
		&self,
		best_hash: B::Hash,
		pre_hash: B::Hash,
		difficulty: ComputeDifficulty,
		proposal: Proposal<B, Proof>,
	) {
		if self.best_hash() != Some(best_hash) {
			self.stop_solving_current();
			return;
		}

		let mut build = self.build.lock();
		*build = Some(MiningBuild { pre_hash: pre_hash.clone(), difficulty, proposal });
	}
	/// Get the version of the mining worker.
	///
	/// This returns type `Version` which can only compare equality. If `Version` is unchanged, then
	/// it can be certain that `best_hash` and `metadata` were not changed.
	pub fn version(&self) -> Version {
		Version(self.version.load(Ordering::SeqCst))
	}

	/// Get the current best hash. `None` if the worker has just started or the client is doing
	/// major syncing.
	pub fn best_hash(&self) -> Option<B::Hash> {
		self.metadata.lock().as_ref().map(|b| b.best_hash)
	}

	pub fn build_hash(&self) -> Option<B::Hash> {
		self.build
			.lock()
			.as_ref()
			.map(|b| b.proposal.block.header().parent_hash())
			.cloned()
	}

	/// Get a copy of the current mining metadata, if available.
	pub fn metadata(&self) -> Option<MiningMetadata<B::Hash>> {
		self.metadata.lock().as_ref().cloned()
	}

	pub fn create_solver(&self) -> Option<ComputeSolver> {
		match self.build.lock().as_ref() {
			Some(x) => {
				let pre_hash = x.pre_hash;

				Some(ComputeSolver::new(self.version(), pre_hash.as_ref().to_vec(), x.difficulty))
			},
			_ => None,
		}
	}

	pub fn is_valid_solver(&self, solver: &Option<Box<ComputeSolver>>) -> bool {
		solver.as_ref().map(|a| a.version) == Some(self.version())
	}

	pub fn ready_to_solve(&self, current_tick: Tick) -> bool {
		match self.metadata.lock().as_ref() {
			Some(x) => {
				if !x.has_tax_votes {
					return true;
				}
				x.parent_tick <= current_tick.saturating_sub(2)
			},
			_ => false,
		}
	}

	pub async fn submit(&mut self, nonce: U256) -> Result<(), Error<B>> {
		let build = match {
			let mut build = self.build.lock();
			// try to take out of option. if not exists, we've moved on
			build.take()
		} {
			Some(x) => x,
			_ => {
				trace!(target: LOG_TARGET, "Unable to submit mined block in compute worker: internal build does not exist",);
				return Ok(());
			},
		};

		self.increment_version();

		let mut block_import = self.block_import.lock();

		block_creator::submit_block::<B, L, Proof>(
			&mut block_import,
			build.proposal,
			&self.justification_sync_link,
			BlockSealDigest::Compute { nonce },
		)
		.await;
		Ok(())
	}
}

impl<B, L, Proof> Clone for MiningHandle<B, L, Proof>
where
	B: BlockT,
	L: sc_consensus::JustificationSyncLink<B>,
{
	fn clone(&self) -> Self {
		Self {
			version: self.version.clone(),
			justification_sync_link: self.justification_sync_link.clone(),
			build: self.build.clone(),
			metadata: self.metadata.clone(),
			block_import: self.block_import.clone(),
		}
	}
}

const LOG_TARGET: &str = "voter::compute::miner";
pub(crate) fn create_compute_solver_task<B, L, Proof>(
	mut worker: MiningHandle<B, L, Proof>,
) -> BoxFuture<'static, ()>
where
	B: BlockT,
	L: sc_consensus::JustificationSyncLink<B> + 'static,
	Proof: Send + 'static,
{
	async move {
		let mut solver: Option<Box<ComputeSolver>> = None;
		loop {
			if !worker.is_valid_solver(&solver) {
				solver = worker.create_solver().map(Box::new);
			}

			if let Some(solver) = solver.as_mut() {
				if let Some(nonce) = solver.check_next() {
					let _ = block_on(worker.submit(nonce.nonce));
				}
			} else {
				tokio::time::sleep(Duration::from_millis(500)).await;
			}
		}
	}
	.boxed()
}

pub fn run_compute_solver_threads<B, L, Proof>(
	task_handle: &TaskManager,
	worker: MiningHandle<B, L, Proof>,
	threads: usize,
) where
	B: BlockT,
	L: sc_consensus::JustificationSyncLink<B> + 'static,
	Proof: Send + 'static,
{
	for _ in 0..threads {
		let worker = worker.clone();
		task_handle.spawn_essential_handle().spawn_essential_blocking(
			"mining-voter",
			Some("block-authoring"),
			create_compute_solver_task(worker),
		);
	}
}

pub fn create_compute_miner<B, C, S, E, SO, L, AccountId>(
	block_import: BoxBlockImport<B>,
	client: Arc<C>,
	aux_client: UlxAux<B, C>,
	select_chain: S,
	mut env: E,
	sync_oracle: SO,
	account_id: AccountId,
	justification_sync_link: L,
	utxo_tracker: Arc<UtxoTracker>,
	max_time_to_build_block: Duration,
) -> (MiningHandle<B, L, <E::Proposer as Proposer<B>>::Proof>, impl Future<Output = ()> + 'static)
where
	B: BlockT,
	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + HeaderBackend<B> + AuxStore + 'static,
	C::Api: BlockSealApis<B, AccountId, BlockSealAuthorityId>
		+ TickApis<B>
		+ NotebookApis<B, NotebookVerifyError>
		+ NotaryApis<B, NotaryRecordT>
		+ BitcoinApis<B, AccountId, BondId, Balance, NumberFor<B>>,
	S: SelectChain<B> + 'static,
	E: Environment<B> + Send + Sync + 'static,
	E::Error: std::fmt::Debug,
	E::Proposer: Proposer<B>,
	SO: SyncOracle + Clone + Send + Sync + 'static,
	L: sc_consensus::JustificationSyncLink<B> + 'static,
	AccountId: Codec + Clone + 'static,
{
	// create a timer that fires whenever there are new blocks, or 500 ms go by
	let mut timer = UntilImportedOrTimeout::new(
		client.import_notification_stream(),
		Duration::from_millis(1000),
	);
	let mining_handle = MiningHandle::new(block_import, justification_sync_link);

	let handle_to_return = mining_handle.clone();

	let ticker = match client.runtime_api().ticker(client.info().genesis_hash) {
		Ok(x) => x,
		Err(err) => {
			panic!("Unable to pull ticker from runtime api: {}", err)
		},
	};

	let task = async move {
		loop {
			if timer.next().await.is_none() {
				// this should occur if the block import notifications completely stop... indicating
				// we should exit
				break;
			}
			if sync_oracle.is_major_syncing() {
				debug!(target: LOG_TARGET, "Skipping proposal due to sync.");
				mining_handle.stop_solving_current();
				continue;
			}

			let best_header = match select_chain.best_chain().await {
				Ok(x) => x,
				Err(err) => {
					warn!(
						target: LOG_TARGET,
						"Unable to pull new block for compute miner. Select best chain error: {}",
						err
					);
					continue;
				},
			};
			let best_hash = best_header.hash();

			if mining_handle.best_hash() != Some(best_hash) {
				let parent_tick = get_tick_digest(best_header.digest()).unwrap_or_default();
				let votes_tick = parent_tick.saturating_sub(1);
				let has_tax_votes = has_votes_at_tick(&client, &best_header, votes_tick);

				mining_handle.on_block(best_hash, has_tax_votes, parent_tick);
			}

			let time = Timestamp::current();
			let tick = ticker.tick_for_time(time.as_millis());
			if !mining_handle.ready_to_solve(tick) {
				continue;
			}

			if mining_handle.build_hash() != Some(best_hash) {
				let notebooks_tick = tick.saturating_sub(1);

				let difficulty = match client.runtime_api().compute_difficulty(best_hash) {
					Ok(x) => x,
					Err(err) => {
						warn!(
							target: LOG_TARGET,
							"Unable to pull new block for compute miner. No difficulty found!! {}", err
						);
						continue;
					},
				};

				let proposal = match propose(
					client.clone(),
					aux_client.clone(),
					&mut env,
					account_id.clone(),
					notebooks_tick,
					time.as_millis(),
					best_hash,
					BlockSealInherentNodeSide::Compute,
					utxo_tracker.clone(),
					max_time_to_build_block,
				)
				.await
				{
					Ok(x) => x,
					Err(err) => {
						warn!(
							target: LOG_TARGET,
							"Unable to propose a new block {}", err
						);
						continue;
					},
				};

				let pre_hash = proposal.block.header().hash();

				mining_handle.start_solving(best_hash, pre_hash, difficulty, proposal);
			}
		}
	};

	(handle_to_return, task)
}

/// A stream that waits for a block import or timeout.
pub struct UntilImportedOrTimeout<Block: BlockT> {
	import_notifications: ImportNotifications<Block>,
	timeout: Duration,
	inner_delay: Option<Delay>,
}

impl<Block: BlockT> UntilImportedOrTimeout<Block> {
	/// Create a new stream using the given import notification and timeout duration.
	pub fn new(import_notifications: ImportNotifications<Block>, timeout: Duration) -> Self {
		Self { import_notifications, timeout, inner_delay: None }
	}
}

impl<Block: BlockT> Stream for UntilImportedOrTimeout<Block> {
	type Item = ();

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
		let mut fire = false;

		loop {
			match Stream::poll_next(Pin::new(&mut self.import_notifications), cx) {
				Poll::Pending => break,
				Poll::Ready(Some(_)) => {
					fire = true;
				},
				Poll::Ready(None) => return Poll::Ready(None),
			}
		}

		let timeout = self.timeout;
		let inner_delay = self.inner_delay.get_or_insert_with(|| Delay::new(timeout));

		match Future::poll(Pin::new(inner_delay), cx) {
			Poll::Pending => (),
			Poll::Ready(()) => {
				fire = true;
			},
		}

		if fire {
			self.inner_delay = None;
			Poll::Ready(Some(()))
		} else {
			Poll::Pending
		}
	}
}
