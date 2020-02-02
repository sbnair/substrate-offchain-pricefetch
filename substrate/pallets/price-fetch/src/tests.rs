#![cfg(test)]

/// tests for this module

// Test cases:
//  1. record_price if called store item in storage
//  2. record_price can only be called from unsigned tx
//  3. with multiple record_price of same sym inserted. On next cycle, the average of the price is calculated
//  4. can fetch for BTC, parse the JSON blob and get a price > 0 out

// use rstd::{prelude::*};
use crate::mock::*;
use support::{assert_ok};

#[test]
fn dispatch_record_price_single_value() {
  new_test_ext().execute_with(|| {
    let sym = b"CCC";
    let remote_src = b"XXX";
    let remote_url = b"http://test.com";

    // Given that nothing in the vector
    let mut test_target = vec![];
    let test_val = 100;

    assert_eq!(<TokenSrcPPMap<TestRuntime>>::get(sym.to_vec()), test_target);

    // We record a src price into the pallet
    let unsigned_call = <Call<TestRuntime>>::record_price(1, (
      sym.to_vec(), remote_src.to_vec(), remote_url.to_vec()), test_val);
    let info = unsigned_call.get_dispatch_info();



    // assert_ok!(PriceFetchModule::record_price(Origin::signed(1), 1, (
    //   sym.to_vec(), remote_src.to_vec(), remote_url.to_vec()), test_val));
    // test_target.push((1, test_val));

    // let result_vec = <TokenSrcPPMap<TestRuntime>>::get(sym.to_vec());
    // assert_eq!(result_vec.len(), 1);
    // // We don't not test for the first element in tuple struct, which is the timestamp
    // assert_eq!(result_vec[0].1, test_val);
  });
}
