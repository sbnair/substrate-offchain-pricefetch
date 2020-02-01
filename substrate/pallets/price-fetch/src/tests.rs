#![cfg(test)]

/// tests for this module

// Test cases:
//  1. record_price if called store item in storage
//  2. record_price can only be called from unsigned tx
//  3. with multiple record_price of same sym inserted. On next cycle, the average of the price is calculated
//  4. can fetch for BTC, parse the JSON blob and get a price > 0 out

use rstd::{prelude::*};
use codec::{Encode, Decode};
use crate::*;
use crate::mock::*;
use support::{assert_ok};

#[test]
fn it_works_for_default_value() {
  new_test_ext().execute_with(|| {
    let sym = b"CCC";
    let remote_src = b"XXX";
    let remote_url = b"http://test.com";

    // Given that nothing in the vector
    let vec = <TokenSrcPPMap<TestRuntime>>::get(sym.to_vec());
    println!("The vec: {:?}", vec);



    // We record a src price into the state machine
    // assert_ok!(PriceFetchModule::record_price(Origin::signed(1), 1, (&sym, &remote_src,
    //   &remote_url), 100));
  });
}
