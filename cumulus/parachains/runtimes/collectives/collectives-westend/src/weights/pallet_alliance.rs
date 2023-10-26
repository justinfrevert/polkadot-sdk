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

//! Autogenerated weights for `pallet_alliance`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-ayothjw6-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("collectives-westend-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-westend-dev
// --wasm-execution=compiled
// --pallet=pallet_alliance
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/collectives/collectives-westend/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_alliance`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_alliance::WeightInfo for WeightInfo<T> {
	/// Storage: `Alliance::Members` (r:1 w:0)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `AllianceMotion::ProposalOf` (r:1 w:1)
	/// Proof: `AllianceMotion::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Proposals` (r:1 w:1)
	/// Proof: `AllianceMotion::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::ProposalCount` (r:1 w:1)
	/// Proof: `AllianceMotion::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Voting` (r:0 w:1)
	/// Proof: `AllianceMotion::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `476 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `6676 + m * (32 ±0) + p * (36 ±0)`
		// Minimum execution time: 28_174_000 picoseconds.
		Weight::from_parts(28_265_273, 0)
			.saturating_add(Weight::from_parts(0, 6676))
			// Standard Error: 149
			.saturating_add(Weight::from_parts(988, 0).saturating_mul(b.into()))
			// Standard Error: 1_562
			.saturating_add(Weight::from_parts(38_035, 0).saturating_mul(m.into()))
			// Standard Error: 1_542
			.saturating_add(Weight::from_parts(178_622, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Alliance::Members` (r:1 w:0)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `AllianceMotion::Voting` (r:1 w:1)
	/// Proof: `AllianceMotion::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `905 + m * (64 ±0)`
		//  Estimated: `6676 + m * (64 ±0)`
		// Minimum execution time: 27_418_000 picoseconds.
		Weight::from_parts(28_547_550, 0)
			.saturating_add(Weight::from_parts(0, 6676))
			// Standard Error: 1_281
			.saturating_add(Weight::from_parts(58_900, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `Alliance::Members` (r:1 w:0)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `AllianceMotion::Voting` (r:1 w:1)
	/// Proof: `AllianceMotion::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Members` (r:1 w:0)
	/// Proof: `AllianceMotion::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Proposals` (r:1 w:1)
	/// Proof: `AllianceMotion::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::ProposalOf` (r:0 w:1)
	/// Proof: `AllianceMotion::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `349 + m * (96 ±0) + p * (36 ±0)`
		//  Estimated: `6676 + m * (97 ±0) + p * (36 ±0)`
		// Minimum execution time: 34_634_000 picoseconds.
		Weight::from_parts(30_932_145, 0)
			.saturating_add(Weight::from_parts(0, 6676))
			// Standard Error: 1_405
			.saturating_add(Weight::from_parts(72_224, 0).saturating_mul(m.into()))
			// Standard Error: 1_370
			.saturating_add(Weight::from_parts(168_173, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 97).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Alliance::Members` (r:1 w:0)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `AllianceMotion::Voting` (r:1 w:1)
	/// Proof: `AllianceMotion::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Members` (r:1 w:0)
	/// Proof: `AllianceMotion::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::ProposalOf` (r:1 w:1)
	/// Proof: `AllianceMotion::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Proposals` (r:1 w:1)
	/// Proof: `AllianceMotion::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `799 + m * (96 ±0) + p * (41 ±0)`
		//  Estimated: `6676 + m * (97 ±0) + p * (40 ±0)`
		// Minimum execution time: 46_869_000 picoseconds.
		Weight::from_parts(40_923_212, 0)
			.saturating_add(Weight::from_parts(0, 6676))
			// Standard Error: 248
			.saturating_add(Weight::from_parts(46, 0).saturating_mul(b.into()))
			// Standard Error: 2_625
			.saturating_add(Weight::from_parts(94_168, 0).saturating_mul(m.into()))
			// Standard Error: 2_559
			.saturating_add(Weight::from_parts(205_865, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 97).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: `Alliance::Members` (r:1 w:0)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `AllianceMotion::Voting` (r:1 w:1)
	/// Proof: `AllianceMotion::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Members` (r:1 w:0)
	/// Proof: `AllianceMotion::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Prime` (r:1 w:0)
	/// Proof: `AllianceMotion::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::ProposalOf` (r:1 w:1)
	/// Proof: `AllianceMotion::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Proposals` (r:1 w:1)
	/// Proof: `AllianceMotion::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Alliance::Rule` (r:0 w:1)
	/// Proof: `Alliance::Rule` (`max_values`: Some(1), `max_size`: Some(87), added: 582, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `555 + m * (96 ±0) + p * (41 ±0)`
		//  Estimated: `6676 + m * (109 ±0) + p * (43 ±0)`
		// Minimum execution time: 47_306_000 picoseconds.
		Weight::from_parts(42_986_284, 0)
			.saturating_add(Weight::from_parts(0, 6676))
			// Standard Error: 4_227
			.saturating_add(Weight::from_parts(136_999, 0).saturating_mul(m.into()))
			// Standard Error: 4_176
			.saturating_add(Weight::from_parts(195_442, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 109).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 43).saturating_mul(p.into()))
	}
	/// Storage: `Alliance::Members` (r:1 w:0)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `AllianceMotion::Voting` (r:1 w:1)
	/// Proof: `AllianceMotion::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Members` (r:1 w:0)
	/// Proof: `AllianceMotion::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Prime` (r:1 w:0)
	/// Proof: `AllianceMotion::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Proposals` (r:1 w:1)
	/// Proof: `AllianceMotion::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::ProposalOf` (r:0 w:1)
	/// Proof: `AllianceMotion::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[5, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `454 + m * (96 ±0) + p * (36 ±0)`
		//  Estimated: `6676 + m * (96 ±0) + p * (36 ±0)`
		// Minimum execution time: 35_896_000 picoseconds.
		Weight::from_parts(31_007_839, 0)
			.saturating_add(Weight::from_parts(0, 6676))
			// Standard Error: 146
			.saturating_add(Weight::from_parts(499, 0).saturating_mul(b.into()))
			// Standard Error: 1_562
			.saturating_add(Weight::from_parts(73_612, 0).saturating_mul(m.into()))
			// Standard Error: 1_506
			.saturating_add(Weight::from_parts(177_870, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 96).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Alliance::Members` (r:2 w:2)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `AllianceMotion::Members` (r:1 w:1)
	/// Proof: `AllianceMotion::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `z` is `[0, 100]`.
	fn init_members(m: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `48`
		//  Estimated: `12362`
		// Minimum execution time: 27_108_000 picoseconds.
		Weight::from_parts(14_902_952, 0)
			.saturating_add(Weight::from_parts(0, 12362))
			// Standard Error: 1_370
			.saturating_add(Weight::from_parts(163_312, 0).saturating_mul(m.into()))
			// Standard Error: 1_354
			.saturating_add(Weight::from_parts(134_431, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Alliance::Members` (r:2 w:2)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `AllianceMotion::Proposals` (r:1 w:0)
	/// Proof: `AllianceMotion::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Alliance::DepositOf` (r:200 w:50)
	/// Proof: `Alliance::DepositOf` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:50 w:50)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AllianceMotion::Members` (r:0 w:1)
	/// Proof: `AllianceMotion::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Prime` (r:0 w:1)
	/// Proof: `AllianceMotion::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 100]`.
	/// The range of component `y` is `[0, 100]`.
	/// The range of component `z` is `[0, 50]`.
	fn disband(x: u32, y: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + x * (52 ±0) + y * (53 ±0) + z * (250 ±0)`
		//  Estimated: `12362 + x * (2539 ±0) + y * (2539 ±0) + z * (2603 ±1)`
		// Minimum execution time: 310_288_000 picoseconds.
		Weight::from_parts(312_847_000, 0)
			.saturating_add(Weight::from_parts(0, 12362))
			// Standard Error: 25_829
			.saturating_add(Weight::from_parts(554_825, 0).saturating_mul(x.into()))
			// Standard Error: 25_705
			.saturating_add(Weight::from_parts(588_691, 0).saturating_mul(y.into()))
			// Standard Error: 51_363
			.saturating_add(Weight::from_parts(13_671_864, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(y.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(z.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(z.into())))
			.saturating_add(Weight::from_parts(0, 2539).saturating_mul(x.into()))
			.saturating_add(Weight::from_parts(0, 2539).saturating_mul(y.into()))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(z.into()))
	}
	/// Storage: `Alliance::Rule` (r:0 w:1)
	/// Proof: `Alliance::Rule` (`max_values`: Some(1), `max_size`: Some(87), added: 582, mode: `MaxEncodedLen`)
	fn set_rule() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_193_000 picoseconds.
		Weight::from_parts(6_440_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Alliance::Announcements` (r:1 w:1)
	/// Proof: `Alliance::Announcements` (`max_values`: Some(1), `max_size`: Some(8702), added: 9197, mode: `MaxEncodedLen`)
	fn announce() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `10187`
		// Minimum execution time: 8_350_000 picoseconds.
		Weight::from_parts(8_675_000, 0)
			.saturating_add(Weight::from_parts(0, 10187))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Alliance::Announcements` (r:1 w:1)
	/// Proof: `Alliance::Announcements` (`max_values`: Some(1), `max_size`: Some(8702), added: 9197, mode: `MaxEncodedLen`)
	fn remove_announcement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `149`
		//  Estimated: `10187`
		// Minimum execution time: 9_268_000 picoseconds.
		Weight::from_parts(9_757_000, 0)
			.saturating_add(Weight::from_parts(0, 10187))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Alliance::Members` (r:3 w:1)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `Alliance::UnscrupulousAccounts` (r:1 w:0)
	/// Proof: `Alliance::UnscrupulousAccounts` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Alliance::DepositOf` (r:0 w:1)
	/// Proof: `Alliance::DepositOf` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	fn join_alliance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `294`
		//  Estimated: `18048`
		// Minimum execution time: 37_756_000 picoseconds.
		Weight::from_parts(38_912_000, 0)
			.saturating_add(Weight::from_parts(0, 18048))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Alliance::Members` (r:3 w:1)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `Alliance::UnscrupulousAccounts` (r:1 w:0)
	/// Proof: `Alliance::UnscrupulousAccounts` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	fn nominate_ally() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `193`
		//  Estimated: `18048`
		// Minimum execution time: 23_340_000 picoseconds.
		Weight::from_parts(24_417_000, 0)
			.saturating_add(Weight::from_parts(0, 18048))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Alliance::Members` (r:2 w:2)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `AllianceMotion::Proposals` (r:1 w:0)
	/// Proof: `AllianceMotion::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Members` (r:0 w:1)
	/// Proof: `AllianceMotion::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Prime` (r:0 w:1)
	/// Proof: `AllianceMotion::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn elevate_ally() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `273`
		//  Estimated: `12362`
		// Minimum execution time: 22_323_000 picoseconds.
		Weight::from_parts(23_152_000, 0)
			.saturating_add(Weight::from_parts(0, 12362))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Alliance::Members` (r:4 w:2)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `AllianceMotion::Proposals` (r:1 w:0)
	/// Proof: `AllianceMotion::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Members` (r:0 w:1)
	/// Proof: `AllianceMotion::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Prime` (r:0 w:1)
	/// Proof: `AllianceMotion::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Alliance::RetiringMembers` (r:0 w:1)
	/// Proof: `Alliance::RetiringMembers` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn give_retirement_notice() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `273`
		//  Estimated: `23734`
		// Minimum execution time: 27_343_000 picoseconds.
		Weight::from_parts(28_590_000, 0)
			.saturating_add(Weight::from_parts(0, 23734))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Alliance::RetiringMembers` (r:1 w:1)
	/// Proof: `Alliance::RetiringMembers` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Alliance::Members` (r:1 w:1)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `Alliance::DepositOf` (r:1 w:1)
	/// Proof: `Alliance::DepositOf` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn retire() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `517`
		//  Estimated: `6676`
		// Minimum execution time: 34_882_000 picoseconds.
		Weight::from_parts(36_031_000, 0)
			.saturating_add(Weight::from_parts(0, 6676))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Alliance::Members` (r:3 w:1)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `AllianceMotion::Proposals` (r:1 w:0)
	/// Proof: `AllianceMotion::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Alliance::DepositOf` (r:1 w:1)
	/// Proof: `Alliance::DepositOf` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Members` (r:0 w:1)
	/// Proof: `AllianceMotion::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Prime` (r:0 w:1)
	/// Proof: `AllianceMotion::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn kick_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `751`
		//  Estimated: `18048`
		// Minimum execution time: 125_529_000 picoseconds.
		Weight::from_parts(129_404_000, 0)
			.saturating_add(Weight::from_parts(0, 18048))
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Alliance::UnscrupulousAccounts` (r:1 w:1)
	/// Proof: `Alliance::UnscrupulousAccounts` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `Alliance::UnscrupulousWebsites` (r:1 w:1)
	/// Proof: `Alliance::UnscrupulousWebsites` (`max_values`: Some(1), `max_size`: Some(25702), added: 26197, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn add_scrupulous_items(n: u32, l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `27187`
		// Minimum execution time: 5_358_000 picoseconds.
		Weight::from_parts(5_480_000, 0)
			.saturating_add(Weight::from_parts(0, 27187))
			// Standard Error: 3_995
			.saturating_add(Weight::from_parts(1_027_103, 0).saturating_mul(n.into()))
			// Standard Error: 1_564
			.saturating_add(Weight::from_parts(71_573, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Alliance::UnscrupulousAccounts` (r:1 w:1)
	/// Proof: `Alliance::UnscrupulousAccounts` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `Alliance::UnscrupulousWebsites` (r:1 w:1)
	/// Proof: `Alliance::UnscrupulousWebsites` (`max_values`: Some(1), `max_size`: Some(25702), added: 26197, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn remove_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + l * (100 ±0) + n * (289 ±0)`
		//  Estimated: `27187`
		// Minimum execution time: 5_414_000 picoseconds.
		Weight::from_parts(5_526_000, 0)
			.saturating_add(Weight::from_parts(0, 27187))
			// Standard Error: 191_933
			.saturating_add(Weight::from_parts(17_796_412, 0).saturating_mul(n.into()))
			// Standard Error: 75_169
			.saturating_add(Weight::from_parts(269_485, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Alliance::Members` (r:3 w:2)
	/// Proof: `Alliance::Members` (`max_values`: None, `max_size`: Some(3211), added: 5686, mode: `MaxEncodedLen`)
	/// Storage: `AllianceMotion::Proposals` (r:1 w:0)
	/// Proof: `AllianceMotion::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Members` (r:0 w:1)
	/// Proof: `AllianceMotion::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AllianceMotion::Prime` (r:0 w:1)
	/// Proof: `AllianceMotion::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn abdicate_fellow_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `273`
		//  Estimated: `18048`
		// Minimum execution time: 27_088_000 picoseconds.
		Weight::from_parts(27_806_000, 0)
			.saturating_add(Weight::from_parts(0, 18048))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
