// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `runtime_common::paras_registrar`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("../polkadot-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ../polkadot-sdk/target/production/polkadot
// benchmark
// pallet
// --chain=../polkadot-chain-spec.json
// --steps
// 50
// --repeat
// 20
// --pallet=runtime_common::paras_registrar
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output
// ./polkadot-weights/
// --header
// ./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `runtime_common::paras_registrar`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_common::paras_registrar::WeightInfo for WeightInfo<T> {
	/// Storage: `Registrar::NextFreeParaId` (r:1 w:1)
	/// Proof: `Registrar::NextFreeParaId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::Paras` (r:1 w:1)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `70`
		//  Estimated: `3535`
		// Minimum execution time: 22_997_000 picoseconds.
		Weight::from_parts(23_646_000, 0)
			.saturating_add(Weight::from_parts(0, 3535))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Registrar::Paras` (r:1 w:1)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:1)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::PvfActiveVoteMap` (r:1 w:1)
	/// Proof: `Paras::PvfActiveVoteMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CodeByHash` (r:1 w:1)
	/// Proof: `Paras::CodeByHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::PvfActiveVoteList` (r:1 w:1)
	/// Proof: `Paras::PvfActiveVoteList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CodeByHashRefs` (r:1 w:1)
	/// Proof: `Paras::CodeByHashRefs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CurrentCodeHash` (r:0 w:1)
	/// Proof: `Paras::CurrentCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpcomingParasGenesis` (r:0 w:1)
	/// Proof: `Paras::UpcomingParasGenesis` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `390`
		//  Estimated: `3855`
		// Minimum execution time: 6_962_183_000 picoseconds.
		Weight::from_parts(7_350_365_000, 0)
			.saturating_add(Weight::from_parts(0, 3855))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Registrar::Paras` (r:1 w:1)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:1)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::PvfActiveVoteMap` (r:1 w:1)
	/// Proof: `Paras::PvfActiveVoteMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CodeByHash` (r:1 w:1)
	/// Proof: `Paras::CodeByHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::PvfActiveVoteList` (r:1 w:1)
	/// Proof: `Paras::PvfActiveVoteList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CodeByHashRefs` (r:1 w:1)
	/// Proof: `Paras::CodeByHashRefs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CurrentCodeHash` (r:0 w:1)
	/// Proof: `Paras::CurrentCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpcomingParasGenesis` (r:0 w:1)
	/// Proof: `Paras::UpcomingParasGenesis` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3745`
		// Minimum execution time: 6_834_072_000 picoseconds.
		Weight::from_parts(7_204_347_000, 0)
			.saturating_add(Weight::from_parts(0, 3745))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Registrar::Paras` (r:1 w:1)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:1)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeHash` (r:1 w:0)
	/// Proof: `Paras::FutureCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ActionsQueue` (r:1 w:1)
	/// Proof: `Paras::ActionsQueue` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:0)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(55), added: 2530, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::PendingSwap` (r:0 w:1)
	/// Proof: `Registrar::PendingSwap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `504`
		//  Estimated: `3969`
		// Minimum execution time: 56_112_000 picoseconds.
		Weight::from_parts(62_891_000, 0)
			.saturating_add(Weight::from_parts(0, 3969))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Registrar::Paras` (r:1 w:0)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:2 w:2)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingSwap` (r:1 w:1)
	/// Proof: `Registrar::PendingSwap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ActionsQueue` (r:1 w:1)
	/// Proof: `Paras::ActionsQueue` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Crowdloan::Funds` (r:2 w:2)
	/// Proof: `Crowdloan::Funds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Slots::Leases` (r:2 w:2)
	/// Proof: `Slots::Leases` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `775`
		//  Estimated: `6715`
		// Minimum execution time: 66_400_000 picoseconds.
		Weight::from_parts(77_994_000, 0)
			.saturating_add(Weight::from_parts(0, 6715))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Paras::FutureCodeHash` (r:1 w:1)
	/// Proof: `Paras::FutureCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeRestrictionSignal` (r:1 w:1)
	/// Proof: `Paras::UpgradeRestrictionSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CurrentCodeHash` (r:1 w:0)
	/// Proof: `Paras::CurrentCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeCooldowns` (r:1 w:1)
	/// Proof: `Paras::UpgradeCooldowns` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::PvfActiveVoteMap` (r:1 w:1)
	/// Proof: `Paras::PvfActiveVoteMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CodeByHash` (r:1 w:1)
	/// Proof: `Paras::CodeByHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::PvfActiveVoteList` (r:1 w:1)
	/// Proof: `Paras::PvfActiveVoteList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CodeByHashRefs` (r:1 w:1)
	/// Proof: `Paras::CodeByHashRefs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 3145728]`.
	fn schedule_code_upgrade(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `3703`
		// Minimum execution time: 33_122_000 picoseconds.
		Weight::from_parts(203_059_387, 0)
			.saturating_add(Weight::from_parts(0, 3703))
			// Standard Error: 13
			.saturating_add(Weight::from_parts(2_018, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: `Paras::Heads` (r:0 w:1)
	/// Proof: `Paras::Heads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 1048576]`.
	fn set_current_head(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_614_000 picoseconds.
		Weight::from_parts(5_881_578, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 1
			.saturating_add(Weight::from_parts(980, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
