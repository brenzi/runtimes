
//! Autogenerated weights for `pallet_bridge_parachains`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `svyatonik-benchmarking`, CPU: `Intel(R) Xeon(R) CPU @ 2.80GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("bh-kusama-local-raw.json")`, DB CACHE: 1024

// Executed Command:
// ../polkadot-sdk/target/production/polkadot-parachain-benchmarks
// benchmark
// pallet
// --chain
// bh-kusama-local-raw.json
// --pallet
// pallet-bridge-parachains
// --extrinsic
// *
// --output=system-parachains/bridge-hubs/bridge-hub-kusama/src/weights
// --no-median-slopes
// --no-min-squares

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge_parachains`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_parachains::WeightInfo for WeightInfo<T> {
	/// Storage: `BridgePolkadotParachains::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotParachains::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotGrandpa::ImportedHeaders` (r:1 w:0)
	/// Proof: `BridgePolkadotGrandpa::ImportedHeaders` (`max_values`: Some(1200), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotParachains::ParasInfo` (r:1 w:1)
	/// Proof: `BridgePolkadotParachains::ParasInfo` (`max_values`: Some(1), `max_size`: Some(60), added: 555, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHashes` (r:1 w:1)
	/// Proof: `BridgePolkadotParachains::ImportedParaHashes` (`max_values`: Some(600), `max_size`: Some(64), added: 1549, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHeads` (r:0 w:1)
	/// Proof: `BridgePolkadotParachains::ImportedParaHeads` (`max_values`: Some(600), `max_size`: Some(196), added: 1681, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 2]`.
	fn submit_parachain_heads_with_n_parachains(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `293`
		//  Estimated: `2543`
		// Minimum execution time: 41_740_000 picoseconds.
		Weight::from_parts(43_231_089, 0)
			.saturating_add(Weight::from_parts(0, 2543))
			// Standard Error: 174_898
			.saturating_add(Weight::from_parts(50_055, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `BridgePolkadotParachains::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotParachains::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotGrandpa::ImportedHeaders` (r:1 w:0)
	/// Proof: `BridgePolkadotGrandpa::ImportedHeaders` (`max_values`: Some(1200), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotParachains::ParasInfo` (r:1 w:1)
	/// Proof: `BridgePolkadotParachains::ParasInfo` (`max_values`: Some(1), `max_size`: Some(60), added: 555, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHashes` (r:1 w:1)
	/// Proof: `BridgePolkadotParachains::ImportedParaHashes` (`max_values`: Some(600), `max_size`: Some(64), added: 1549, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHeads` (r:0 w:1)
	/// Proof: `BridgePolkadotParachains::ImportedParaHeads` (`max_values`: Some(600), `max_size`: Some(196), added: 1681, mode: `MaxEncodedLen`)
	fn submit_parachain_heads_with_1kb_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `293`
		//  Estimated: `2543`
		// Minimum execution time: 43_838_000 picoseconds.
		Weight::from_parts(44_567_000, 0)
			.saturating_add(Weight::from_parts(0, 2543))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `BridgePolkadotParachains::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotParachains::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotGrandpa::ImportedHeaders` (r:1 w:0)
	/// Proof: `BridgePolkadotGrandpa::ImportedHeaders` (`max_values`: Some(1200), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotParachains::ParasInfo` (r:1 w:1)
	/// Proof: `BridgePolkadotParachains::ParasInfo` (`max_values`: Some(1), `max_size`: Some(60), added: 555, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHashes` (r:1 w:1)
	/// Proof: `BridgePolkadotParachains::ImportedParaHashes` (`max_values`: Some(600), `max_size`: Some(64), added: 1549, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHeads` (r:0 w:1)
	/// Proof: `BridgePolkadotParachains::ImportedParaHeads` (`max_values`: Some(600), `max_size`: Some(196), added: 1681, mode: `MaxEncodedLen`)
	fn submit_parachain_heads_with_16kb_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `293`
		//  Estimated: `2543`
		// Minimum execution time: 83_675_000 picoseconds.
		Weight::from_parts(84_895_000, 0)
			.saturating_add(Weight::from_parts(0, 2543))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
