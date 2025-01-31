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

//! Autogenerated weights for pallet_multisig
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-04, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
// --execution=wasm
// --wasm-execution=compiled
// --extrinsic=*
// --heap-pages=4096
// --output=./runtimes/spiritnet/src/weights/pallet_multisig.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_imports)]
#![allow(clippy::as_conversions)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		Weight::from_parts(8_364_882 as u64, 0)
			// Standard Error: 79
			.saturating_add(Weight::from_parts(849 as u64, 0).saturating_mul(z as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Proof: Multisig Multisigs (max_values: None, max_size: Some(2198), added: 4673, mode: MaxEncodedLen)
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		Weight::from_parts(20_285_811 as u64, 0)
			// Standard Error: 5_408
			.saturating_add(Weight::from_parts(80_065 as u64, 0).saturating_mul(s as u64))
			// Standard Error: 33
			.saturating_add(Weight::from_parts(986 as u64, 0).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Proof: Multisig Multisigs (max_values: None, max_size: Some(2198), added: 4673, mode: MaxEncodedLen)
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		Weight::from_parts(12_705_025 as u64, 0)
			// Standard Error: 30_300
			.saturating_add(Weight::from_parts(20_431 as u64, 0).saturating_mul(s as u64))
			// Standard Error: 186
			.saturating_add(Weight::from_parts(3_373 as u64, 0).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Proof: Multisig Multisigs (max_values: None, max_size: Some(2198), added: 4673, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		Weight::from_parts(20_642_060 as u64, 0)
			// Standard Error: 5_511
			.saturating_add(Weight::from_parts(104_693 as u64, 0).saturating_mul(s as u64))
			// Standard Error: 34
			.saturating_add(Weight::from_parts(1_132 as u64, 0).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Proof: Multisig Multisigs (max_values: None, max_size: Some(2198), added: 4673, mode: MaxEncodedLen)
	fn approve_as_multi_create(s: u32, ) -> Weight {
		Weight::from_parts(17_036_891 as u64, 0)
			// Standard Error: 4_200
			.saturating_add(Weight::from_parts(122_554 as u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Proof: Multisig Multisigs (max_values: None, max_size: Some(2198), added: 4673, mode: MaxEncodedLen)
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		Weight::from_parts(12_421_505 as u64, 0)
			// Standard Error: 7_770
			.saturating_add(Weight::from_parts(81_668 as u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Proof: Multisig Multisigs (max_values: None, max_size: Some(2198), added: 4673, mode: MaxEncodedLen)
	fn cancel_as_multi(s: u32, ) -> Weight {
		Weight::from_parts(19_681_233 as u64, 0)
			// Standard Error: 4_292
			.saturating_add(Weight::from_parts(70_945 as u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
