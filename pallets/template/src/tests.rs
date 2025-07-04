use crate::{mock::*, Error, Event, Something};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(Template::register_metadata(RuntimeOrigin::signed(1), 
			BoundedVec::try_from(b"dataset-001".to_vec()).unwrap(), 
			BoundedVec::try_from(b"{\"energy\": 17.3}".to_vec()).unwrap()));
		// Read pallet storage and assert an expected result.
		assert_eq!(Something::<Test>::get(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

