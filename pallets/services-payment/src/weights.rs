// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_services_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `tomasz-XPS-15-9520`, CPU: `12th Gen Intel(R) Core(TM) i7-12700H`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_services_payment
// --extrinsic
// *
// --chain=dev
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// tmp/pallet_services_payment.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_services_payment.
pub trait WeightInfo {
	fn purchase_credits() -> Weight;
	fn set_block_production_credits() -> Weight;
	fn set_given_free_credits() -> Weight;
	fn set_refund_address() -> Weight;
	fn set_max_core_price() -> Weight;
	fn on_container_author_noted() -> Weight;
	fn on_collators_assigned() -> Weight;
	fn set_max_tip() -> Weight;
}

/// Weights for pallet_services_payment using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn purchase_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `155`
		//  Estimated: `6196`
		// Minimum execution time: 78_059_000 picoseconds.
		Weight::from_parts(78_662_000, 6196)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn set_block_production_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_769_000 picoseconds.
		Weight::from_parts(13_036_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::GivenFreeCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::GivenFreeCredits` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	fn set_given_free_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_288_000 picoseconds.
		Weight::from_parts(7_550_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:0)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ServicesPayment::RefundAddress` (r:0 w:1)
	/// Proof: `ServicesPayment::RefundAddress` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_refund_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3660`
		// Minimum execution time: 9_065_000 picoseconds.
		Weight::from_parts(9_720_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:0)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ServicesPayment::MaxCorePrice` (r:0 w:1)
	/// Proof: `ServicesPayment::MaxCorePrice` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn set_max_core_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3660`
		// Minimum execution time: 8_773_000 picoseconds.
		Weight::from_parts(9_211_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:1 w:0)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn on_container_author_noted() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `3593`
		// Minimum execution time: 13_759_000 picoseconds.
		Weight::from_parts(14_206_000, 3593)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::CollatorAssignmentCredits` (r:1 w:0)
	/// Proof: `ServicesPayment::CollatorAssignmentCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn on_collators_assigned() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `3593`
		// Minimum execution time: 13_924_000 picoseconds.
		Weight::from_parts(14_441_000, 3593)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::MaxTip` (r:0 w:1)
	/// Proof: `ServicesPayment::MaxTip` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn set_max_tip() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_157_000 picoseconds.
		Weight::from_parts(4_576_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn purchase_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `155`
		//  Estimated: `6196`
		// Minimum execution time: 78_059_000 picoseconds.
		Weight::from_parts(78_662_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn set_block_production_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_769_000 picoseconds.
		Weight::from_parts(13_036_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::GivenFreeCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::GivenFreeCredits` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	fn set_given_free_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_288_000 picoseconds.
		Weight::from_parts(7_550_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:0)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ServicesPayment::RefundAddress` (r:0 w:1)
	/// Proof: `ServicesPayment::RefundAddress` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_refund_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3660`
		// Minimum execution time: 9_065_000 picoseconds.
		Weight::from_parts(9_720_000, 3660)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:0)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ServicesPayment::MaxCorePrice` (r:0 w:1)
	/// Proof: `ServicesPayment::MaxCorePrice` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn set_max_core_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3660`
		// Minimum execution time: 8_773_000 picoseconds.
		Weight::from_parts(9_211_000, 3660)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:1 w:0)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn on_container_author_noted() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `3593`
		// Minimum execution time: 13_759_000 picoseconds.
		Weight::from_parts(14_206_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::CollatorAssignmentCredits` (r:1 w:0)
	/// Proof: `ServicesPayment::CollatorAssignmentCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn on_collators_assigned() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `3593`
		// Minimum execution time: 13_924_000 picoseconds.
		Weight::from_parts(14_441_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::MaxTip` (r:0 w:1)
	/// Proof: `ServicesPayment::MaxTip` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn set_max_tip() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_157_000 picoseconds.
		Weight::from_parts(4_576_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
