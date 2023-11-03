
//! Autogenerated weights for `pallet_treasury`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `caribe`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("encointer-rococo-local-dev"), DB CACHE: 1024

// Executed Command:
// target/release/encointer-collator
// benchmark
// pallet
// --chain=encointer-rococo-local-dev
// --steps=50
// --repeat=20
// --pallet=pallet_treasury
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=polkadot-parachains/encointer-runtime/src/weights/pallet_treasury.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_treasury`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_treasury::WeightInfo for WeightInfo<T> {
	fn spend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 366_000 picoseconds.
		Weight::from_parts(420_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Treasury ProposalCount (r:1 w:1)
	/// Proof: Treasury ProposalCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Treasury Proposals (r:0 w:1)
	/// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	fn propose_spend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `107`
		//  Estimated: `1489`
		// Minimum execution time: 44_556_000 picoseconds.
		Weight::from_parts(45_578_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Treasury Proposals (r:1 w:1)
	/// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn reject_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `265`
		//  Estimated: `3593`
		// Minimum execution time: 45_779_000 picoseconds.
		Weight::from_parts(47_134_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Treasury Proposals (r:1 w:0)
	/// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	/// Storage: Treasury Approvals (r:1 w:1)
	/// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(41), added: 536, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 9]`.
	fn approve_proposal(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `118 + p * (31 ±0)`
		//  Estimated: `3573`
		// Minimum execution time: 9_093_000 picoseconds.
		Weight::from_parts(11_820_833, 0)
			.saturating_add(Weight::from_parts(0, 3573))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Treasury Approvals (r:1 w:1)
	/// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(41), added: 536, mode: MaxEncodedLen)
	fn remove_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `90`
		//  Estimated: `1526`
		// Minimum execution time: 5_560_000 picoseconds.
		Weight::from_parts(5_797_000, 0)
			.saturating_add(Weight::from_parts(0, 1526))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Treasury Deactivated (r:1 w:1)
	/// Proof: Treasury Deactivated (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:1)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Treasury Approvals (r:1 w:1)
	/// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(41), added: 536, mode: MaxEncodedLen)
	/// Storage: Treasury Proposals (r:10 w:10)
	/// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	/// Storage: System Account (r:20 w:20)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 10]`.
	fn on_initialize_proposals(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `39 + p * (232 ±0)`
		//  Estimated: `1526 + p * (5206 ±0)`
		// Minimum execution time: 26_465_000 picoseconds.
		Weight::from_parts(34_535_717, 0)
			.saturating_add(Weight::from_parts(0, 1526))
			// Standard Error: 69_073
			.saturating_add(Weight::from_parts(43_631_242, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 5206).saturating_mul(p.into()))
	}
}