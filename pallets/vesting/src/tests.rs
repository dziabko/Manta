use super::{Event as PalletEvent, *};
use frame_support::{assert_noop, assert_ok};
use mock::{Event as MockEvent, *};

#[test]
fn alice_vesting_for_bob_should_work() {
	ExtBuilder::default()
		.existential_deposit(1)
		.build()
		.execute_with(|| {
			let unvested = 100;
			assert_ok!(MantaVesting::vested_transfer(
				Origin::signed(ALICE),
				BOB,
				unvested
			));
			assert_eq!(Balances::free_balance(ALICE), 10_000 - unvested);
			assert_eq!(Balances::free_balance(BOB), unvested);
			assert_eq!(VestingBalances::<Test>::get(BOB), Some(unvested));

			// Now Bob cannot claim any token.
			assert_noop!(
				MantaVesting::vest(Origin::signed(BOB)),
				Error::<Test>::ClaimTooEarly,
			);

			// Check event
			System::assert_has_event(MockEvent::MantaVesting(PalletEvent::VestingUpdated(
				BOB, unvested,
			)));

			run_to_block(3);
			let now = VestingSchedule::<Test>::get()[0].1 * 1000 + 1;
			Timestamp::set_timestamp(now);

			assert_ok!(MantaVesting::vest(Origin::signed(BOB)));
			assert_eq!(Balances::free_balance(BOB), unvested);

			// BOB cannot transfer more than 34 tokens.
			// Bacause rest of 66 is locked now.
			let vested = 34;
			// Check event
			System::assert_has_event(MockEvent::MantaVesting(PalletEvent::VestingUpdated(
				BOB,
				unvested - vested,
			)));

			assert_noop!(
				Balances::transfer(Origin::signed(BOB), ALICE, vested + 1),
				pallet_balances::Error::<Test, _>::LiquidityRestrictions,
			);

			assert_ok!(Balances::transfer(Origin::signed(BOB), ALICE, vested));
			assert_eq!(Balances::free_balance(ALICE), 10_000 - unvested + vested);
			assert_eq!(Balances::free_balance(BOB), unvested - vested);

			// Ensure Bob can claim all tokens once vesting is done.
			let now = VestingSchedule::<Test>::get()[6].1 * 1000 + 1;
			Timestamp::set_timestamp(now);

			assert_ok!(MantaVesting::vest(Origin::signed(BOB)));
			assert_eq!(Balances::free_balance(BOB), unvested - vested);

			// Check vested done event
			System::assert_has_event(MockEvent::MantaVesting(PalletEvent::VestingCompleted(BOB)));

			// Now, Bob can transfer all his tokens.
			assert_ok!(Balances::transfer(
				Origin::signed(BOB),
				ALICE,
				unvested - vested
			));
			assert_eq!(Balances::free_balance(ALICE), 10_000);
			assert_eq!(Balances::free_balance(BOB), 0);

			// Ensure vesting info is removed once vesting is done.
			assert_eq!(VestingBalances::<Test>::get(BOB), None);
		});
}

#[test]
fn alice_vesting_for_bob_claim_slowly_should_work() {
	ExtBuilder::default()
		.existential_deposit(1)
		.build()
		.execute_with(|| {
			let unvested = 100;
			assert_ok!(MantaVesting::vested_transfer(
				Origin::signed(ALICE),
				BOB,
				unvested
			));
			assert_eq!(Balances::free_balance(ALICE), 10_000 - unvested);
			assert_eq!(Balances::free_balance(BOB), unvested);
			assert_eq!(VestingBalances::<Test>::get(BOB), Some(unvested));

			// Now Bob cannot claim any token.
			assert_noop!(
				MantaVesting::vest(Origin::signed(BOB)),
				Error::<Test>::ClaimTooEarly,
			);

			// Check event
			System::assert_has_event(MockEvent::MantaVesting(PalletEvent::VestingUpdated(
				BOB, unvested,
			)));

			// Ensure Bob can cliam his token once the 4th round is done.
			let now = VestingSchedule::<Test>::get()[3].1 * 1000 + 1;
			Timestamp::set_timestamp(now);

			assert_ok!(MantaVesting::vest(Origin::signed(BOB)));
			assert_eq!(Balances::free_balance(BOB), unvested);

			// BOB cannot transfer more than 67 tokens.
			// Bacause rest of 33 is locked now.
			let vested = 67;
			assert_noop!(
				Balances::transfer(Origin::signed(BOB), ALICE, vested + 1),
				pallet_balances::Error::<Test, _>::LiquidityRestrictions,
			);

			assert_ok!(Balances::transfer(Origin::signed(BOB), ALICE, vested));
			assert_eq!(Balances::free_balance(ALICE), 10_000 - unvested + vested);
			assert_eq!(Balances::free_balance(BOB), unvested - vested);
		});
}

#[test]
fn alice_vesting_for_bob_claim_arbitrarily_should_work() {
	ExtBuilder::default()
		.existential_deposit(1)
		.build()
		.execute_with(|| {
			let unvested = 100;
			assert_ok!(MantaVesting::vested_transfer(
				Origin::signed(ALICE),
				BOB,
				unvested
			));
			assert_eq!(Balances::free_balance(ALICE), 10_000 - unvested);
			assert_eq!(Balances::free_balance(BOB), unvested);
			assert_eq!(VestingBalances::<Test>::get(BOB), Some(unvested));

			run_to_block(3);
			let now = VestingSchedule::<Test>::get()[0].1 * 1000 + 1;
			Timestamp::set_timestamp(now);

			assert_ok!(MantaVesting::vest(Origin::signed(BOB)));
			assert_eq!(Balances::free_balance(BOB), unvested);

			// BOB cannot transfer more than 34 tokens.
			// Bacause rest of 66 is locked now.
			let vested = 34;
			// Check event
			System::assert_has_event(MockEvent::MantaVesting(PalletEvent::VestingUpdated(
				BOB,
				unvested - vested,
			)));

			assert_noop!(
				Balances::transfer(Origin::signed(BOB), ALICE, vested + 1),
				pallet_balances::Error::<Test, _>::LiquidityRestrictions,
			);

			assert_ok!(Balances::transfer(Origin::signed(BOB), ALICE, vested));
			assert_eq!(Balances::free_balance(ALICE), 10_000 - unvested + vested);
			assert_eq!(Balances::free_balance(BOB), unvested - vested);

			// Now release tokens for the 6th round.
			let now = VestingSchedule::<Test>::get()[5].1 * 1000 + 1;
			Timestamp::set_timestamp(now);

			assert_ok!(MantaVesting::vest(Origin::signed(BOB)));
			// BOB cannot transfer more than 34 tokens.
			// Bacause rest of 11 is locked now.
			let vested = 55;
			assert_noop!(
				Balances::transfer(Origin::signed(BOB), ALICE, vested + 1),
				pallet_balances::Error::<Test, _>::LiquidityRestrictions,
			);

			// Check event
			System::assert_has_event(MockEvent::MantaVesting(PalletEvent::VestingUpdated(
				BOB, 11,
			)));
			assert_eq!(Balances::free_balance(BOB), vested + 11);

			assert_ok!(Balances::transfer(Origin::signed(BOB), ALICE, vested));
			assert_eq!(Balances::free_balance(ALICE), 10_000 - 11);
			assert_eq!(Balances::free_balance(BOB), 11);
		});
}

#[test]
fn vesting_complete_should_work() {
	ExtBuilder::default()
		.existential_deposit(1)
		.build()
		.execute_with(|| {
			let unvested = 100;
			assert_ok!(MantaVesting::vested_transfer(
				Origin::signed(ALICE),
				BOB,
				unvested
			));
			assert_eq!(Balances::free_balance(ALICE), 10_000 - unvested);
			assert_eq!(VestingBalances::<Test>::get(BOB), Some(unvested));

			// Now Bob cannot claim any token.
			assert_noop!(
				MantaVesting::vest(Origin::signed(BOB)),
				Error::<Test>::ClaimTooEarly,
			);

			// Check event
			System::assert_has_event(MockEvent::MantaVesting(PalletEvent::VestingUpdated(
				BOB, unvested,
			)));

			// Now Bob cannot transfer locked tokens.
			assert_noop!(
				Balances::transfer(Origin::signed(BOB), ALICE, 1),
				pallet_balances::Error::<Test, _>::LiquidityRestrictions,
			);

			// Ensure Bob can claim all tokens once vesting is done.
			let now = VestingSchedule::<Test>::get()[6].1 * 1000 + 1;
			Timestamp::set_timestamp(now);

			assert_ok!(MantaVesting::vest(Origin::signed(BOB)));
			assert_eq!(Balances::free_balance(BOB), unvested);

			// Check vested done event
			System::assert_has_event(MockEvent::MantaVesting(PalletEvent::VestingCompleted(BOB)));
			let vested = unvested;

			// Now, Bob can transfer all his tokens.
			assert_ok!(Balances::transfer(Origin::signed(BOB), ALICE, vested));
			assert_eq!(Balances::free_balance(ALICE), 10_000);
			assert_eq!(Balances::free_balance(BOB), 0);

			// Ensure vesting info is removed once vesting is done.
			assert_eq!(VestingBalances::<Test>::get(BOB), None);
		});
}

#[test]
fn update_vesting_schedule_should_work() {
	use chrono::prelude::*;

	let default_schedule: [(Percent, i32, u32, u32, u32, u32, u32, &'static str); 7] = [
		(
			Percent::from_percent(34),
			2021,
			11,
			08,
			0,
			0,
			0,
			"2021-11-08 00:00:00",
		),
		(
			Percent::from_percent(11),
			2021,
			11,
			10,
			0,
			0,
			0,
			"2021-11-10 00:00:00",
		),
		(
			Percent::from_percent(11),
			2022,
			01,
			05,
			0,
			0,
			0,
			"2022-01-05 00:00:00",
		),
		(
			Percent::from_percent(11),
			2022,
			03,
			02,
			0,
			0,
			0,
			"2022-03-02 00:00:00",
		),
		(
			Percent::from_percent(11),
			2022,
			04,
			27,
			0,
			0,
			0,
			"2022-04-27 00:00:00",
		),
		(
			Percent::from_percent(11),
			2022,
			06,
			22,
			0,
			0,
			0,
			"2022-06-22 00:00:00",
		),
		(
			Percent::from_percent(11),
			2022,
			08,
			17,
			0,
			0,
			0,
			"2022-08-17 00:00:00",
		),
	];
	ExtBuilder::default()
		.existential_deposit(1)
		.build()
		.execute_with(|| {
			// Check current schedule.
			let schedule = VestingSchedule::<Test>::get();
			assert_eq!(schedule.len(), MaxReserves::get() as usize);

			//Check percentage.
			assert_eq!(
				schedule
					.iter()
					.map(|(p, _)| p)
					.fold(Percent::from_percent(0), |acc, p| acc.saturating_add(*p)),
				Percent::from_percent(100)
			);

			for ((p, s), ds) in schedule.iter().zip(default_schedule.iter()) {
				let dt = Utc.ymd(ds.1, ds.2, ds.3).and_hms(ds.4, ds.5, ds.6);

				// Check each percentage is correct.
				assert_eq!(ds.0, *p);
				// Check datetime is correct.
				assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), ds.7);
				// Check timestamp is correct.
				assert_eq!(dt.timestamp() as u64, *s);
			}

			// Cannot update the length of schedule is bigger than 7 or smaller than 7.
			let wrong_length_schedule: BoundedVec<u64, MaxReserves> =
				BoundedVec::try_from(vec![1, 2, 3, 4, 5, 6, 7, 8]).unwrap_or_default();
			assert_noop!(
				MantaVesting::update_vesting_schedule(Origin::root(), wrong_length_schedule),
				Error::<Test>::InvalidScheduleLength,
			);

			let wrong_length_schedule: BoundedVec<u64, MaxReserves> =
				BoundedVec::try_from(vec![1, 2, 3, 4, 5, 6]).unwrap_or_default();
			assert_noop!(
				MantaVesting::update_vesting_schedule(Origin::root(), wrong_length_schedule),
				Error::<Test>::InvalidScheduleLength,
			);

			// The new schedule should be a sorted array.
			let invalid_schedule: BoundedVec<u64, MaxReserves> =
				BoundedVec::try_from(vec![1, 2, 9, 4, 8, 6, 7]).unwrap_or_default();
			assert_noop!(
				MantaVesting::update_vesting_schedule(Origin::root(), invalid_schedule),
				Error::<Test>::UnsortedSchedule,
			);

			let now = VestingSchedule::<Test>::get()[6].1 * 1000 + 1;
			Timestamp::set_timestamp(now);

			// The new schedule should not be past time.
			let invalid_schedule: BoundedVec<u64, MaxReserves> = BoundedVec::try_from(vec![
				1636311600, 1636311601, 1636311602, 1636311603, 1636311604, 1636311605, 1636311606,
			])
			.unwrap_or_default();
			assert_noop!(
				MantaVesting::update_vesting_schedule(Origin::root(), invalid_schedule),
				Error::<Test>::InvalidTimestamp,
			);

			let now = VestingSchedule::<Test>::get()[0].1 * 1000 + 1;
			Timestamp::set_timestamp(now);

			let new_schedule = BoundedVec::try_from(
				VestingSchedule::<Test>::get()
					.iter()
					.map(|(_, s)| s + 1)
					.collect::<Vec<u64>>(),
			)
			.unwrap_or_default();
			assert_ok!(MantaVesting::update_vesting_schedule(
				Origin::root(),
				new_schedule.clone()
			));
			// Check storage
			assert_eq!(
				VestingSchedule::<Test>::get()
					.iter()
					.map(|(_, s)| *s)
					.collect::<Vec<u64>>(),
				*new_schedule
			);
			// Check event
			System::assert_has_event(MockEvent::MantaVesting(
				PalletEvent::VestingScheduleUpdated(new_schedule),
			));
		});
}
