// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_xcm_bridge_hub_router`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-vmdtonbz-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("asset-hub-rococo-dev")`, DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/polkadot-sdk/.git/.artifacts/bench.json
// --pallet=pallet_xcm_bridge_hub_router
// --chain=asset-hub-rococo-dev
// --header=./cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/assets/asset-hub-rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_xcm_bridge_hub_router`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm_bridge_hub_router::WeightInfo for WeightInfo<T> {
	/// Storage: UNKNOWN KEY `0x48297505634037ef48c848c99c0b1f1b` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x48297505634037ef48c848c99c0b1f1b` (r:1 w:0)
	/// Storage: `XcmpQueue::InboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::InboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ToRococoXcmRouter::Bridge` (r:1 w:1)
	/// Proof: `ToRococoXcmRouter::Bridge` (`max_values`: Some(1), `max_size`: Some(17), added: 512, mode: `MaxEncodedLen`)
	fn on_initialize_when_non_congested() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `265`
		//  Estimated: `3730`
		// Minimum execution time: 9_084_000 picoseconds.
		Weight::from_parts(9_441_000, 0)
			.saturating_add(Weight::from_parts(0, 3730))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: UNKNOWN KEY `0x48297505634037ef48c848c99c0b1f1b` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x48297505634037ef48c848c99c0b1f1b` (r:1 w:0)
	/// Storage: `XcmpQueue::InboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::InboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_initialize_when_congested() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `202`
		//  Estimated: `3667`
		// Minimum execution time: 5_971_000 picoseconds.
		Weight::from_parts(6_260_000, 0)
			.saturating_add(Weight::from_parts(0, 3667))
			.saturating_add(T::DbWeight::get().reads(3))
	}
	/// Storage: `ToRococoXcmRouter::Bridge` (r:1 w:1)
	/// Proof: `ToRococoXcmRouter::Bridge` (`max_values`: Some(1), `max_size`: Some(17), added: 512, mode: `MaxEncodedLen`)
	fn report_bridge_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `117`
		//  Estimated: `1502`
		// Minimum execution time: 10_231_000 picoseconds.
		Weight::from_parts(10_861_000, 0)
			.saturating_add(Weight::from_parts(0, 1502))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: UNKNOWN KEY `0x48297505634037ef48c848c99c0b1f1b` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x48297505634037ef48c848c99c0b1f1b` (r:1 w:0)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0973fe64c85043ba1c965cbc38eb63c7` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x0973fe64c85043ba1c965cbc38eb63c7` (r:1 w:0)
	/// Storage: `ToRococoXcmRouter::Bridge` (r:1 w:1)
	/// Proof: `ToRococoXcmRouter::Bridge` (`max_values`: Some(1), `max_size`: Some(17), added: 512, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `XcmpQueue::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::RelevantMessagingState` (r:1 w:0)
	/// Proof: `ParachainSystem::RelevantMessagingState` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::InboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::InboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::OutboundXcmpMessages` (r:0 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpMessages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn send_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `478`
		//  Estimated: `3943`
		// Minimum execution time: 53_966_000 picoseconds.
		Weight::from_parts(55_224_000, 0)
			.saturating_add(Weight::from_parts(0, 3943))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
