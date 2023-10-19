
//! Autogenerated weights for `pallet_sassafras`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-19, STEPS: `10`, REPEAT: `3`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `behemoth`, CPU: `AMD Ryzen Threadripper 3970X 32-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_sassafras
// --extrinsic
// *
// --steps
// 10
// --repeat
// 3
// --output
// weights.rs
// --template
// substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_sassafras`.
pub trait WeightInfo {
	fn submit_tickets(x: u32, ) -> Weight;
	fn plan_config_change() -> Weight;
	fn load_ring_context() -> Weight;
	fn update_ring_verifier(x: u32, ) -> Weight;
	fn sort_segments(x: u32, y: u32, ) -> Weight;
}

/// Weights for `pallet_sassafras` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Sassafras::RingVerifierData` (r:1 w:0)
	/// Proof: `Sassafras::RingVerifierData` (`max_values`: Some(1), `max_size`: Some(422), added: 917, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextAuthorities` (r:1 w:0)
	/// Proof: `Sassafras::NextAuthorities` (`max_values`: Some(1), `max_size`: Some(331), added: 826, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextEpochConfig` (r:1 w:0)
	/// Proof: `Sassafras::NextEpochConfig` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextRandomness` (r:1 w:0)
	/// Proof: `Sassafras::NextRandomness` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::EpochIndex` (r:1 w:0)
	/// Proof: `Sassafras::EpochIndex` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::TicketsData` (r:20 w:20)
	/// Proof: `Sassafras::TicketsData` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::TicketsMeta` (r:1 w:1)
	/// Proof: `Sassafras::TicketsMeta` (`max_values`: Some(1), `max_size`: Some(12), added: 507, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextTicketsSegments` (r:0 w:1)
	/// Proof: `Sassafras::NextTicketsSegments` (`max_values`: None, `max_size`: Some(57606), added: 60081, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 20]`.
	fn submit_tickets(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1230`
		//  Estimated: `1907 + x * (2559 ±0)`
		// Minimum execution time: 24_262_186_000 picoseconds.
		Weight::from_parts(13_727_608_869, 1907)
			// Standard Error: 56_792_244
			.saturating_add(Weight::from_parts(11_313_333_616, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 2559).saturating_mul(x.into()))
	}
	/// Storage: `Sassafras::PendingEpochConfigChange` (r:0 w:1)
	/// Proof: `Sassafras::PendingEpochConfigChange` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn plan_config_change() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_148_000 picoseconds.
		Weight::from_parts(4_478_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Sassafras::RingContext` (r:1 w:0)
	/// Proof: `Sassafras::RingContext` (`max_values`: Some(1), `max_size`: Some(295412), added: 295907, mode: `MaxEncodedLen`)
	fn load_ring_context() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295540`
		//  Estimated: `296897`
		// Minimum execution time: 20_784_762_000 picoseconds.
		Weight::from_parts(20_893_379_000, 296897)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `Sassafras::RingContext` (r:1 w:0)
	/// Proof: `Sassafras::RingContext` (`max_values`: Some(1), `max_size`: Some(295412), added: 295907, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::RingVerifierData` (r:0 w:1)
	/// Proof: `Sassafras::RingVerifierData` (`max_values`: Some(1), `max_size`: Some(422), added: 917, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 20]`.
	fn update_ring_verifier(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295540`
		//  Estimated: `296897`
		// Minimum execution time: 52_377_380_000 picoseconds.
		Weight::from_parts(53_881_364_161, 296897)
			// Standard Error: 48_651_666
			.saturating_add(Weight::from_parts(93_138_482, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Sassafras::NextTicketsSegments` (r:3 w:3)
	/// Proof: `Sassafras::NextTicketsSegments` (`max_values`: None, `max_size`: Some(57606), added: 60081, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::TicketsIds` (r:0 w:3600)
	/// Proof: `Sassafras::TicketsIds` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 1800]`.
	/// The range of component `y` is `[1, 2]`.
	fn sort_segments(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1373 + y * (246 ±0)`
		//  Estimated: `25043 + x * (16 ±3) + y * (67910 ±5_319)`
		// Minimum execution time: 19_438_000 picoseconds.
		Weight::from_parts(3_015_904_180, 25043)
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(y.into())))
			.saturating_add(T::DbWeight::get().writes(2161_u64))
			.saturating_add(Weight::from_parts(0, 16).saturating_mul(x.into()))
			.saturating_add(Weight::from_parts(0, 67910).saturating_mul(y.into()))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Sassafras::RingVerifierData` (r:1 w:0)
	/// Proof: `Sassafras::RingVerifierData` (`max_values`: Some(1), `max_size`: Some(422), added: 917, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextAuthorities` (r:1 w:0)
	/// Proof: `Sassafras::NextAuthorities` (`max_values`: Some(1), `max_size`: Some(331), added: 826, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextEpochConfig` (r:1 w:0)
	/// Proof: `Sassafras::NextEpochConfig` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextRandomness` (r:1 w:0)
	/// Proof: `Sassafras::NextRandomness` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::EpochIndex` (r:1 w:0)
	/// Proof: `Sassafras::EpochIndex` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::TicketsData` (r:20 w:20)
	/// Proof: `Sassafras::TicketsData` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::TicketsMeta` (r:1 w:1)
	/// Proof: `Sassafras::TicketsMeta` (`max_values`: Some(1), `max_size`: Some(12), added: 507, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextTicketsSegments` (r:0 w:1)
	/// Proof: `Sassafras::NextTicketsSegments` (`max_values`: None, `max_size`: Some(57606), added: 60081, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 20]`.
	fn submit_tickets(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1230`
		//  Estimated: `1907 + x * (2559 ±0)`
		// Minimum execution time: 24_262_186_000 picoseconds.
		Weight::from_parts(13_727_608_869, 1907)
			// Standard Error: 56_792_244
			.saturating_add(Weight::from_parts(11_313_333_616, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 2559).saturating_mul(x.into()))
	}
	/// Storage: `Sassafras::PendingEpochConfigChange` (r:0 w:1)
	/// Proof: `Sassafras::PendingEpochConfigChange` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn plan_config_change() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_148_000 picoseconds.
		Weight::from_parts(4_478_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Sassafras::RingContext` (r:1 w:0)
	/// Proof: `Sassafras::RingContext` (`max_values`: Some(1), `max_size`: Some(295412), added: 295907, mode: `MaxEncodedLen`)
	fn load_ring_context() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295540`
		//  Estimated: `296897`
		// Minimum execution time: 20_784_762_000 picoseconds.
		Weight::from_parts(20_893_379_000, 296897)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: `Sassafras::RingContext` (r:1 w:0)
	/// Proof: `Sassafras::RingContext` (`max_values`: Some(1), `max_size`: Some(295412), added: 295907, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::RingVerifierData` (r:0 w:1)
	/// Proof: `Sassafras::RingVerifierData` (`max_values`: Some(1), `max_size`: Some(422), added: 917, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 20]`.
	fn update_ring_verifier(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295540`
		//  Estimated: `296897`
		// Minimum execution time: 52_377_380_000 picoseconds.
		Weight::from_parts(53_881_364_161, 296897)
			// Standard Error: 48_651_666
			.saturating_add(Weight::from_parts(93_138_482, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Sassafras::NextTicketsSegments` (r:3 w:3)
	/// Proof: `Sassafras::NextTicketsSegments` (`max_values`: None, `max_size`: Some(57606), added: 60081, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::TicketsIds` (r:0 w:3600)
	/// Proof: `Sassafras::TicketsIds` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 1800]`.
	/// The range of component `y` is `[1, 2]`.
	fn sort_segments(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1373 + y * (246 ±0)`
		//  Estimated: `25043 + x * (16 ±3) + y * (67910 ±5_319)`
		// Minimum execution time: 19_438_000 picoseconds.
		Weight::from_parts(3_015_904_180, 25043)
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(y.into())))
			.saturating_add(RocksDbWeight::get().writes(2161_u64))
			.saturating_add(Weight::from_parts(0, 16).saturating_mul(x.into()))
			.saturating_add(Weight::from_parts(0, 67910).saturating_mul(y.into()))
	}
}
