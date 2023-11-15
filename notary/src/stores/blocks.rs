use std::collections::{BTreeMap, BTreeSet};

use chrono::Utc;
use sp_core::H256;
use sqlx::{FromRow, PgConnection};

use ulx_notary_primitives::{ensure, BlockVoteEligibility, BlockVoteSource};

use crate::Error;

pub struct BlocksStore;

#[derive(Debug, FromRow)]
pub struct BlockRow {
	pub block_hash: Vec<u8>,
	pub parent_hash: Vec<u8>,
	pub block_number: i32,
	pub block_vote_minimum: String,
	pub block_vote_source: BlockVoteSource,
	pub this_notary_notebook_number: Option<i32>,
	pub parent_voting_key: Vec<u8>,
	pub is_finalized: bool,
	pub finalized_time: Option<chrono::DateTime<Utc>>,
	pub received_time: chrono::DateTime<Utc>,
}

impl BlocksStore {
	pub(crate) async fn get_specifications(
		&self,
		db: &mut PgConnection,
		block_hashes: &BTreeSet<H256>,
	) -> anyhow::Result<BTreeMap<H256, BlockVoteEligibility>, crate::Error> {
		let mut map = BTreeMap::new();
		let block_hashes_vec = block_hashes.iter().map(|h| h.0.to_vec()).collect::<Vec<_>>();
		let rows = sqlx::query!(
			r#"
		SELECT block_hash, block_vote_minimum, block_vote_source FROM blocks where block_hash = ANY($1)
		"#,
			&block_hashes_vec
		)
		.fetch_all(db)
		.await?;

		for row in rows {
			map.insert(
				H256::from_slice(&row.block_hash[..]),
				BlockVoteEligibility {
					allowed_sources: row.block_vote_source.into(),
					minimum: row.block_vote_minimum.parse::<u128>().map_err(|e| {
						Error::InternalError(format!("Unable to parse minimum: {:?}", e))
					})?,
				},
			);
		}
		Ok(map)
	}

	pub async fn lock(db: &mut PgConnection) -> anyhow::Result<(), crate::Error> {
		let _ = sqlx::query_scalar!(
			"SELECT key FROM block_sync_lock where key = 1 FOR UPDATE NOWAIT LIMIT 1"
		)
		.fetch_one(db)
		.await?;

		Ok(())
	}
	pub async fn get_latest_finalized_block_number(
		db: &mut PgConnection,
	) -> anyhow::Result<u32, crate::Error> {
		let row = sqlx::query_scalar!(
			r#"
		SELECT COALESCE(MAX(block_number), 0) FROM blocks WHERE is_finalized=true;
		"#,
		)
		.fetch_one(db)
		.await?
		.unwrap_or_default();
		Ok(row as u32)
	}

	pub async fn get_latest_block_number(
		db: &mut PgConnection,
	) -> anyhow::Result<u32, crate::Error> {
		let row = sqlx::query_scalar!(
			r#"
		SELECT COALESCE(MAX(block_number), 0) FROM blocks;
		"#,
		)
		.fetch_one(db)
		.await?
		.unwrap_or_default();
		Ok(row as u32)
	}

	pub async fn get_by_hash(
		db: &mut PgConnection,
		block_hash: H256,
	) -> anyhow::Result<BlockRow, Error> {
		let row = sqlx::query_as!(
			BlockRow,
			r#"
		SELECT * FROM blocks WHERE block_hash=$1 LIMIT 1;
		"#,
			block_hash.0.to_vec()
		)
		.fetch_one(db)
		.await?;
		Ok(row)
	}

	pub async fn has_block(db: &mut PgConnection, block_hash: H256) -> anyhow::Result<bool, Error> {
		let row = sqlx::query_scalar!(
			r#"
			SELECT 1 as true FROM blocks WHERE block_hash=$1 LIMIT 1;
			"#,
			block_hash.0.to_vec()
		)
		.fetch_optional(db)
		.await?;
		Ok(row.is_some())
	}

	pub async fn get_all_voting_keys(
		db: &mut PgConnection,
		notebook_number: u32,
	) -> anyhow::Result<Vec<H256>, Error> {
		let rows = sqlx::query_scalar!(
			r#"
		SELECT parent_voting_key FROM blocks WHERE this_notary_notebook_number=$1 AND parent_voting_key IS NOT NULL;
		"#,
			notebook_number as i32
		)
		.fetch_all(db)
		.await?;
		let rows = rows.iter().map(|a| H256::from_slice(&a[..])).collect::<Vec<_>>();
		Ok(rows)
	}

	pub async fn get_parent_hash(
		db: &mut PgConnection,
		block_hash: H256,
	) -> anyhow::Result<H256, Error> {
		let row = sqlx::query_scalar!(
			r#"
		SELECT parent_hash FROM blocks WHERE block_hash=$1 LIMIT 1;
		"#,
			block_hash.0.to_vec()
		)
		.fetch_one(db)
		.await?;
		Ok(H256::from_slice(&row[..]))
	}

	pub async fn record_finalized(
		db: &mut PgConnection,
		block_hash: H256,
	) -> anyhow::Result<(), crate::Error> {
		let res = sqlx::query!(
			r#"
			UPDATE blocks SET finalized_time=$1, is_finalized=true
			WHERE block_hash = $2
		"#,
			Utc::now(),
			block_hash.0.to_vec(),
		)
		.execute(db)
		.await?;

		ensure!(
			res.rows_affected() == 1,
			Error::InternalError("Unable to insert block".to_string())
		);
		Ok(())
	}

	pub async fn record(
		db: &mut PgConnection,
		block_number: u32,
		block_hash: H256,
		parent_hash: H256,
		vote_eligibility: BlockVoteEligibility,
		parent_voting_key: H256,
		this_notary_notebook_number: Option<u32>,
	) -> anyhow::Result<(), Error> {
		let res = sqlx::query!(
			r#"
			INSERT INTO blocks (block_hash, block_number, parent_hash, block_vote_minimum, block_vote_source, received_time, is_finalized,
				parent_voting_key, this_notary_notebook_number)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
		"#,
			block_hash.0.to_vec(),
			block_number as i32,
			parent_hash.0.to_vec(),
			vote_eligibility.minimum.to_string(),
			vote_eligibility.allowed_sources as i32,
			Utc::now(),
			false,
			parent_voting_key.0.to_vec(),
			this_notary_notebook_number.map(|n| n as i32),
		)
		.execute(db)
		.await?;

		ensure!(
			res.rows_affected() == 1,
			Error::InternalError("Unable to insert block".to_string())
		);
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use sp_core::H256;
	use sqlx::PgPool;

	use ulx_notary_primitives::{BlockVoteEligibility, BlockVoteSource};

	use crate::stores::blocks::BlocksStore;

	#[sqlx::test]
	async fn test_storage(pool: PgPool) -> anyhow::Result<()> {
		{
			let mut tx = pool.begin().await?;
			BlocksStore::record(
				&mut *tx,
				0,
				H256::from_slice(&[1u8; 32]),
				H256::zero(),
				BlockVoteEligibility::new(100, BlockVoteSource::Tax),
				H256::from_slice(&[1u8; 32]),
				None,
			)
			.await?;
			BlocksStore::record(
				&mut *tx,
				1,
				H256::from_slice(&[2u8; 32]),
				H256::from_slice(&[1u8; 32]),
				BlockVoteEligibility::new(100, BlockVoteSource::Tax),
				H256::from_slice(&[1u8; 32]),
				None,
			)
			.await?;
			BlocksStore::record(
				&mut *tx,
				2,
				H256::from_slice(&[3u8; 32]),
				H256::from_slice(&[2u8; 32]),
				BlockVoteEligibility::new(100, BlockVoteSource::Tax),
				H256::from_slice(&[1u8; 32]),
				None,
			)
			.await?;
			BlocksStore::record_finalized(&mut *tx, H256::from_slice(&[3u8; 32])).await?;
			tx.commit().await?;
		}
		{
			let mut tx = pool.begin().await?;
			let result = BlocksStore::get_latest_finalized_block_number(&mut *tx).await?;
			assert_eq!(result, 2);
			tx.commit().await?;
		}

		Ok(())
	}
	#[sqlx::test]
	async fn test_cant_overwrite(pool: PgPool) -> anyhow::Result<()> {
		let mut tx = pool.begin().await?;
		BlocksStore::record(
			&mut *tx,
			1,
			H256::from_slice(&[1u8; 32]),
			H256::zero(),
			BlockVoteEligibility::new(100, BlockVoteSource::Tax),
			H256::from_slice(&[1u8; 32]),
			None,
		)
		.await?;
		assert!(BlocksStore::record(
			&mut *tx,
			1,
			H256::from_slice(&[1u8; 32]),
			H256::from_slice(&[1u8; 32]),
			BlockVoteEligibility::new(100, BlockVoteSource::Tax),
			H256::from_slice(&[1u8; 32]),
			None
		)
		.await
		.unwrap_err()
		.to_string()
		.contains("duplicate key"));

		Ok(())
	}
}
