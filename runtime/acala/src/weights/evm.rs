//! Autogenerated weights for evm
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2020-12-08, STEPS: [1, ], REPEAT: 1, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB
//! CACHE: 128

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

use sp_std::marker::PhantomData;

/// Weight functions for evm.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_evm::WeightInfo for WeightInfo<T> {
	fn add_storage_quota() -> Weight {
		(299_052_000 as Weight)
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn remove_storage_quota() -> Weight {
		(257_267_000 as Weight)
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn request_transfer_maintainer() -> Weight {
		(241_867_000 as Weight)
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn cancel_transfer_maintainer() -> Weight {
		(239_974_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn confirm_transfer_maintainer() -> Weight {
		(257_354_000 as Weight)
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn reject_transfer_maintainer() -> Weight {
		(260_551_000 as Weight)
			.saturating_add(DbWeight::get().reads(6 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
}
