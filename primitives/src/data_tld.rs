use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::TypeInfo, Deserialize, Serialize};
use sp_debug_derive::RuntimeDebug;

#[derive(
	Clone,
	PartialEq,
	Eq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	Serialize,
	Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub enum DataTLD {
	Analytics,
	Automotive,
	Bikes,
	Business,
	Cars,
	Communication,
	Entertainment,
	Finance,
	Flights,
	Health,
	Hotels,
	Jobs,
	News,
	RealEstate,
	Restaurants,
	Shopping,
	Sports,
	Transportation,
	Travel,
	Weather,
}
