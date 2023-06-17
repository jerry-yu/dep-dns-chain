// imports
use crate::mock::{self, Balances, System};
use frame_system::Origin;
use frame_system::Event;
use frame_support::{assert_ok, assert_noop};
use crate::mock::new_test_ext;
use crate::mock::Dns;
use crate::mock::Test;
use pallet_balances::Error as BalancesError;

#[test]
fn it_should_register_domain_successfully() {
	new_test_ext().execute_with(|| {
		// Set the current block number
		System::set_block_number(1);

		let account_id = 1;
		let domain = "example".to_string();
		let domain_alias = vec!["alias".to_string()];
		let owner_info = "Owner Information".to_string();

		// Provide initial balance for account
		Balances::deposit_creating(&account_id, 1000);

		// Dispatch a signed extrinsic for domain registration
		assert_ok!(Dns::register_domain(Origin::signed(account_id), domain.clone(), owner_info.clone(), domain_alias.clone()));

		// Test event has been emitted
		let event = Event::Dns(Event::<Test>::Rigistration { name: domain.clone(), owner: account_id, expire: 11 });
		assert!(System::events().iter().any(|a| a.event == event));
	});
}

#[test]
fn it_should_not_register_domain_if_exists() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let account_id = 1;
		let domain = "example".to_string();
		let domain_alias = vec!["alias".to_string()];
		let owner_info = "Owner Information".to_string();

		Balances::deposit_creating(&account_id, 2000);

		assert_ok!(Dns::register_domain(Origin::signed(account_id), domain.clone(), owner_info.clone(), domain_alias.clone()));
		assert_noop!(
			Dns::register_domain(Origin::signed(account_id), domain.clone(), owner_info.clone(), domain_alias.clone()),
			BalancesError::<Test>::Existed
		);
	});
}

#[test]
fn it_should_transfer_ownership() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let old_owner = 1;
		let new_owner = 2;
		let domain = "example".to_string();
		let domain_alias = vec!["alias".to_string()];
		let owner_info = "Owner Information".to_string();

		Balances::deposit_creating(&old_owner, 1000);
		assert_ok!(Dns::register_domain(Origin::signed(old_owner), domain.clone(), owner_info.clone(), domain_alias.clone()));
		
		assert_ok!(Dns::transfer_ownershit(Origin::signed(old_owner), domain.clone(), new_owner));

		// Test event has been emitted
		let event = Event::Dns(Event::<Test>::OwnerShip { name: domain.clone(), old_owner, recipient: new_owner });
		assert!(System::events().iter().any(|a| a.event == event));
	});
}

// More tests...
