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

//! Autogenerated weights for `pallet_tips`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 33.0.0
//! DATE: 2024-06-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `eyrie-7`, CPU: `Intel(R) Core(TM) i7-7700 CPU @ 3.60GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --template=.maintain/runtime-weight-template.hbs
// --header=HEADER-GPL
// --wasm-execution=compiled
// --heap-pages=4096
// --steps=50
// --repeat=20
// --chain=spiritnet-dev
// --pallet=pallet-tips
// --extrinsic=*
// --output=./runtimes/spiritnet/src/weights/pallet_tips.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_imports)]
#![allow(clippy::as_conversions)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_tips`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tips::WeightInfo for WeightInfo<T> {
	/// Storage: `Tips::Reasons` (r:1 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `r` is `[0, 16384]`.
	fn report_awesome(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3469`
		// Minimum execution time: 27_952_000 picoseconds.
		Weight::from_parts(28_728_213, 0)
			.saturating_add(Weight::from_parts(0, 3469))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_556, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Reasons` (r:0 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn retract_tip() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `221`
		//  Estimated: `3686`
		// Minimum execution time: 25_478_000 picoseconds.
		Weight::from_parts(25_777_000, 0)
			.saturating_add(Weight::from_parts(0, 3686))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `TipsMembership::Members` (r:1 w:0)
	/// Proof: `TipsMembership::Members` (`max_values`: Some(1), `max_size`: Some(673), added: 1168, mode: `MaxEncodedLen`)
	/// Storage: `Tips::Reasons` (r:1 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Tips` (r:0 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `r` is `[0, 16384]`.
	/// The range of component `t` is `[1, 21]`.
	fn tip_new(r: u32, t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `141 + t * (32 ±0)`
		//  Estimated: `3606 + t * (32 ±0)`
		// Minimum execution time: 19_804_000 picoseconds.
		Weight::from_parts(17_926_991, 0)
			.saturating_add(Weight::from_parts(0, 3606))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(1_423, 0).saturating_mul(r.into()))
			// Standard Error: 5_788
			.saturating_add(Weight::from_parts(162_652, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(t.into()))
	}
	/// Storage: `TipsMembership::Members` (r:1 w:0)
	/// Proof: `TipsMembership::Members` (`max_values`: Some(1), `max_size`: Some(673), added: 1168, mode: `MaxEncodedLen`)
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `t` is `[1, 21]`.
	fn tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366 + t * (80 ±0)`
		//  Estimated: `3831 + t * (80 ±0)`
		// Minimum execution time: 15_693_000 picoseconds.
		Weight::from_parts(15_940_899, 0)
			.saturating_add(Weight::from_parts(0, 3831))
			// Standard Error: 1_182
			.saturating_add(Weight::from_parts(126_181, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(t.into()))
	}
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TipsMembership::Members` (r:1 w:0)
	/// Proof: `TipsMembership::Members` (`max_values`: Some(1), `max_size`: Some(673), added: 1168, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Tips::Reasons` (r:0 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `t` is `[1, 21]`.
	fn close_tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `473 + t * (80 ±0)`
		//  Estimated: `6204 + t * (80 ±0)`
		// Minimum execution time: 55_188_000 picoseconds.
		Weight::from_parts(56_227_331, 0)
			.saturating_add(Weight::from_parts(0, 6204))
			// Standard Error: 4_354
			.saturating_add(Weight::from_parts(135_011, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(t.into()))
	}
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Reasons` (r:0 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `t` is `[1, 21]`.
	fn slash_tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `3734`
		// Minimum execution time: 14_238_000 picoseconds.
		Weight::from_parts(14_632_080, 0)
			.saturating_add(Weight::from_parts(0, 3734))
			// Standard Error: 717
			.saturating_add(Weight::from_parts(9_426, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_report_awesome() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3469
		);
	}
	#[test]
	fn test_retract_tip() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3686
		);
	}
	#[test]
	fn test_tip_new() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3606
		);
	}
	#[test]
	fn test_tip() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3831
		);
	}
	#[test]
	fn test_close_tip() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6204
		);
	}
	#[test]
	fn test_slash_tip() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 3734
		);
	}
}
