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

//! Autogenerated weights for ctype
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-18
//! STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `rust-2`, CPU: `12th Gen Intel(R) Core(TM) i9-12900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --template=.maintain/weight-template.hbs
// --header=HEADER-GPL
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --steps=50
// --repeat=20
// --chain=dev
// --pallet=ctype
// --extrinsic=*
// --output=./pallets/ctype/src/default_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_imports)]
#![allow(clippy::as_conversions)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for ctype.
pub trait WeightInfo {
	fn add(l: u32, ) -> Weight;
	fn set_block_number() -> Weight;
}

/// Weights for ctype using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Ctype Ctypes (r:1 w:1)
	/// Proof: Ctype Ctypes (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5242880]`.
	fn add(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `616`
		//  Estimated: `7777`
		// Minimum execution time: 48_042 nanoseconds.
		Weight::from_parts(48_883_000, 7777)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_195, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Ctype Ctypes (r:1 w:1)
	/// Proof: Ctype Ctypes (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	fn set_block_number() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `151`
		//  Estimated: `2563`
		// Minimum execution time: 10_067 nanoseconds.
		Weight::from_parts(13_231_000, 2563)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Ctype Ctypes (r:1 w:1)
	/// Proof: Ctype Ctypes (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5242880]`.
	fn add(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `616`
		//  Estimated: `7777`
		// Minimum execution time: 48_042 nanoseconds.
		Weight::from_parts(48_883_000, 7777)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_195, 0).saturating_mul(l.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Ctype Ctypes (r:1 w:1)
	/// Proof: Ctype Ctypes (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	fn set_block_number() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `151`
		//  Estimated: `2563`
		// Minimum execution time: 10_067 nanoseconds.
		Weight::from_parts(13_231_000, 2563)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
