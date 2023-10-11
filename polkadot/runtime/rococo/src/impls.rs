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

use crate::xcm_config;
use frame_support::pallet_prelude::DispatchResult;
use frame_system::RawOrigin;
use pallet_identity::OnReapIdentity;
use primitives::Balance;
use rococo_runtime_constants::currency::*;
use sp_std::{marker::PhantomData, prelude::*};
use xcm::{latest::prelude::*, /* v2::OriginKind, */ VersionedMultiLocation, VersionedXcm};
use xcm_executor::traits::TransactAsset;

/// Type that implements `OnReapIdentity` that will send the deposit needed to store the same
/// information on a parachain, sends the deposit there, and then updates it.
pub struct ToParachainIdentityReaper<T>(PhantomData<T>);
impl<T> ToParachainIdentityReaper<T> {
	/// Calculate the balance needed on the remote chain based on the `IdentityInfo` and `Subs` on
	/// this chain. The total includes:
	///
	/// - Identity basic deposit
	/// - Extra fields deposit
	/// - Sub accounts deposit
	/// - 2x existential deposit (1 for account existence, 1 such that the user can transact)
	///
	/// This implementation is speculative and subject to change based on the remote chain's
	/// configuration.
	fn calculate_remote_deposit(fields: u32, subs: u32) -> Balance {
		// remote deposit constants
		let para_basic_deposit = 1000 * CENTS / 100;
		let para_field_deposit = 250 * CENTS / 100;
		let para_sub_account_deposit = 200 * CENTS / 100;
		let para_existential_deposit = EXISTENTIAL_DEPOSIT / 10;

		// pallet deposits
		let id_deposit =
			para_basic_deposit.saturating_add(para_field_deposit.saturating_mul(fields as Balance));
		let subs_deposit = para_sub_account_deposit.saturating_mul(subs as Balance);

		id_deposit
			.saturating_add(subs_deposit)
			.saturating_add(para_existential_deposit.saturating_mul(2))
	}
}

impl<AccountId, T> OnReapIdentity<AccountId> for ToParachainIdentityReaper<T>
where
	T: frame_system::Config + pallet_xcm::Config,
	AccountId: Into<[u8; 32]> + Clone,
{
	fn on_reap_identity(who: &AccountId, fields: u32, subs: u32) -> DispatchResult {
		let total_to_send = Self::calculate_remote_deposit(fields, subs);

		// define asset / destination from relay perspective
		let roc: MultiAssets =
			vec![MultiAsset { id: Concrete(Here.into_location()), fun: Fungible(total_to_send) }]
				.into();
		// todo: people chain para id
		let destination: MultiLocation = MultiLocation::new(0, Parachain(1000));

		// Do `check_out` accounting since the XCM Executor's `InitiateTeleport` doesn't support
		// unpaid teleports.

		// check out
		xcm_config::LocalAssetTransactor::can_check_out(
			&destination,
			&roc.inner().first().unwrap(), // <- safe unwrap since we just set `roc`.
			// not used in AssetTransactor
			&XcmContext { origin: None, message_id: [0; 32], topic: None },
		)
		.map_err(|_| pallet_xcm::Error::<T>::InvalidAsset)?;
		xcm_config::LocalAssetTransactor::check_out(
			&destination,
			&roc.inner().first().unwrap(), // <- safe unwrap since we just set `roc`.
			// not used in AssetTransactor
			&XcmContext { origin: None, message_id: [0; 32], topic: None },
		);

		// reanchor
		let roc_reanchored: MultiAssets = vec![MultiAsset {
			id: Concrete(MultiLocation::new(1, Here)),
			fun: Fungible(total_to_send),
		}]
		.into();

		// Actual program to execute on People Chain.
		let program: Xcm<()> = Xcm(vec![
			// Unpaid as this is constructed by the system, once per user. The user shouldn't have
			// their balance reduced by teleport fees for the favor of migrating.
			UnpaidExecution { weight_limit: Unlimited, check_origin: None },
			// Receive the asset into holding.
			ReceiveTeleportedAsset(roc_reanchored),
			// Deposit into the user's account.
			DepositAsset {
				assets: Wild(AllCounted(1)),
				beneficiary: Junction::AccountId32 { network: None, id: who.clone().into() }
					.into_location()
					.into(),
			},
			// // Poke the deposit to reserve the appropriate amount on the parachain.
			// Transact {
			// 	origin_kind: OriginKind::Superuser,
			// 	require_weight_at_most: Weight {ref_time: 2_000_000_000, proof_size: 16_384},
			// 	// Need People Chain runtime to encode call.
			// 	call: DoubleEncoded { encoded: /* pallet_identity::poke_deposit(who) */ },
			// },
		]);

		// send
		let _ = <pallet_xcm::Pallet<T>>::send(
			RawOrigin::Root.into(),
			Box::new(VersionedMultiLocation::V3(destination)),
			Box::new(VersionedXcm::V3(program)),
		)?;
		Ok(())
	}
}
