// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2024 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Autogenerated weights for `pallet_did_lookup`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 33.0.0
//! DATE: 2024-11-11, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `rust-2`, CPU: `12th Gen Intel(R) Core(TM) i9-12900K`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/debug/kilt-parachain
// benchmark
// pallet
// --heap-pages=4096
// --chain=dev
// --pallet=pallet-did-lookup
// --extrinsic=*
// --steps=2
// --repeat=1
// --default-pov-mode=ignored
// --header=HEADER-GPL
// --template=.maintain/runtime-weight-template.hbs
// --output=./runtimes/peregrine/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_imports)]
#![allow(clippy::as_conversions)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_did_lookup`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_did_lookup::WeightInfo for WeightInfo<T> {
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `Ignored`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(301), added: 2776, mode: `Ignored`)
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `Ignored`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:2)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `Ignored`)
	/// Storage: `Migration::MigratedKeys` (r:0 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `Ignored`)
	fn associate_account_multisig_sr25519() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `462`
		//  Estimated: `990`
		// Minimum execution time: 8_231_783_000 picoseconds.
		Weight::from_parts(8_231_783_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `Ignored`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(301), added: 2776, mode: `Ignored`)
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `Ignored`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:2)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `Ignored`)
	/// Storage: `Migration::MigratedKeys` (r:0 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `Ignored`)
	fn associate_account_multisig_ed25519() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `462`
		//  Estimated: `990`
		// Minimum execution time: 3_532_242_000 picoseconds.
		Weight::from_parts(3_532_242_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `Ignored`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(301), added: 2776, mode: `Ignored`)
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `Ignored`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:2)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `Ignored`)
	/// Storage: `Migration::MigratedKeys` (r:0 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `Ignored`)
	fn associate_account_multisig_ecdsa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `462`
		//  Estimated: `990`
		// Minimum execution time: 4_624_007_000 picoseconds.
		Weight::from_parts(4_624_007_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `Ignored`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(301), added: 2776, mode: `Ignored`)
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `Ignored`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:2)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `Ignored`)
	/// Storage: `Migration::MigratedKeys` (r:0 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `Ignored`)
	fn associate_eth_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `449`
		//  Estimated: `990`
		// Minimum execution time: 7_807_837_000 picoseconds.
		Weight::from_parts(7_807_837_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `Ignored`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(301), added: 2776, mode: `Ignored`)
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `Ignored`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:2)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `Ignored`)
	/// Storage: `Migration::MigratedKeys` (r:0 w:1)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `Ignored`)
	fn associate_sender() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `462`
		//  Estimated: `990`
		// Minimum execution time: 3_061_394_000 picoseconds.
		Weight::from_parts(3_061_394_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `Ignored`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `Ignored`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `Ignored`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(301), added: 2776, mode: `Ignored`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:1)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `Ignored`)
	fn remove_sender_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `540`
		//  Estimated: `990`
		// Minimum execution time: 1_769_975_000 picoseconds.
		Weight::from_parts(1_769_975_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `Ignored`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `Ignored`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `Ignored`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(301), added: 2776, mode: `Ignored`)
	/// Storage: `DidLookup::ConnectedAccounts` (r:0 w:1)
	/// Proof: `DidLookup::ConnectedAccounts` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `Ignored`)
	fn remove_account_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `540`
		//  Estimated: `990`
		// Minimum execution time: 1_514_759_000 picoseconds.
		Weight::from_parts(1_514_759_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `Ignored`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `Ignored`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `Ignored`)
	/// Storage: `Balances::Holds` (r:2 w:2)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(301), added: 2776, mode: `Ignored`)
	fn change_deposit_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `647`
		//  Estimated: `990`
		// Minimum execution time: 2_512_929_000 picoseconds.
		Weight::from_parts(2_512_929_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `DidLookup::ConnectedDids` (r:1 w:1)
	/// Proof: `DidLookup::ConnectedDids` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `Ignored`)
	/// Storage: `Migration::MigratedKeys` (r:1 w:0)
	/// Proof: `Migration::MigratedKeys` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `Ignored`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `Ignored`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(301), added: 2776, mode: `Ignored`)
	fn update_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `540`
		//  Estimated: `990`
		// Minimum execution time: 2_369_856_000 picoseconds.
		Weight::from_parts(2_369_856_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_associate_account_multisig_sr25519() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 990
		);
	}
	#[test]
	fn test_associate_account_multisig_ed25519() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 990
		);
	}
	#[test]
	fn test_associate_account_multisig_ecdsa() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 990
		);
	}
	#[test]
	fn test_associate_eth_account() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 990
		);
	}
	#[test]
	fn test_associate_sender() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 990
		);
	}
	#[test]
	fn test_remove_sender_association() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 990
		);
	}
	#[test]
	fn test_remove_account_association() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 990
		);
	}
	#[test]
	fn test_change_deposit_owner() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 990
		);
	}
	#[test]
	fn test_update_deposit() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 990
		);
	}
}
