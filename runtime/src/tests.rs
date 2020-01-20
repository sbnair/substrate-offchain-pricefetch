/// tests for this module

// Test cases:
//  1. record_price if called store item in storage
//  2. record_price can only be called from unsigned tx
//  3. with multiple record_price of same sym inserted. On next cycle, the average of the price is calculated
//  4. can fetch for BTC, parse the JSON blob and get a price > 0 out

use crate::price_fetch::{self, Module, Trait};
use primitives::{H256};
use support::{dispatch, impl_outer_origin, impl_outer_dispatch, assert_ok,
  parameter_types, weights::Weight};
use sp_runtime::{
  app_crypto::{AppPublic, RuntimeAppPublic},
  traits::{BlakeTwo256, IdentityLookup},
  testing::{Header, TestXt},
  Perbill
};
use codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

impl_outer_origin! {
  pub enum Origin for Test {}
}

impl_outer_dispatch! {
  pub enum Call for Test where origin: Origin {
    price_fetch::PriceFetchModule,
  }
}

// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;

pub type AccountId = u64;

mod sp_core_crypto_dummy {
	use primitives::crypto::*;
    use codec::{Encode, Decode};

	/// Dummy cryptography. Doesn't do anything.
	#[derive(Clone, Hash, Default, Eq, PartialEq, Encode, Decode, Debug, Ord, PartialOrd)]
	pub struct Dummy;

    impl std::fmt::Display for Dummy {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(fmt, "DummyCrypto")
        }
    }

    impl sp_runtime::app_crypto::RuntimePublic for Dummy {
        type Signature = Self;

        fn all(_key_type: KeyTypeId) -> crate::Vec<Self> {
            vec![Dummy]
        }

        fn generate_pair(key_type: KeyTypeId, seed: Option<Vec<u8>>) -> Self {
            Dummy
        }

        fn sign<M: AsRef<[u8]>>(&self, key_type: KeyTypeId, msg: &M) -> Option<Self::Signature> {
            Some(self.clone())
        }

        fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool {
            true
        }
    }

	impl AsRef<[u8]> for Dummy {
		fn as_ref(&self) -> &[u8] { &b""[..] }
	}

	impl AsMut<[u8]> for Dummy {
		fn as_mut(&mut self) -> &mut [u8] {
			unsafe {
				#[allow(mutable_transmutes)]
				//sp_std::mem::transmute::<_, &'static mut [u8]>(&b""[..])
                unimplemented!()
			}
		}
	}

	impl CryptoType for Dummy {
		type Pair = Dummy;
	}

	impl Derive for Dummy {}

	impl Public for Dummy {
		fn from_slice(_: &[u8]) -> Self { Self }
		#[cfg(feature = "std")]
		fn to_raw_vec(&self) -> Vec<u8> { vec![] }
		fn as_slice(&self) -> &[u8] { b"" }
	}

	impl Pair for Dummy {
		type Public = Dummy;
		type Seed = Dummy;
		type Signature = Dummy;
		type DeriveError = ();
		#[cfg(feature = "std")]
		fn generate_with_phrase(_: Option<&str>) -> (Self, String, Self::Seed) { Default::default() }
		#[cfg(feature = "std")]
		fn from_phrase(_: &str, _: Option<&str>)
			-> Result<(Self, Self::Seed), SecretStringError>
		{
			Ok(Default::default())
		}
		fn derive<
			Iter: Iterator<Item=DeriveJunction>,
		>(&self, _: Iter, _: Option<Dummy>) -> Result<(Self, Option<Dummy>), Self::DeriveError> { Ok((Self, None)) }
		fn from_seed(_: &Self::Seed) -> Self { Self }
		fn from_seed_slice(_: &[u8]) -> Result<Self, SecretStringError> { Ok(Self) }
		fn sign(&self, _: &[u8]) -> Self::Signature { Self }
		fn verify<M: AsRef<[u8]>>(_: &Self::Signature, _: M, _: &Self::Public) -> bool { true }
		fn verify_weak<P: AsRef<[u8]>, M: AsRef<[u8]>>(_: &[u8], _: M, _: P) -> bool { true }
		fn public(&self) -> Self::Public { Self }
		fn to_raw_vec(&self) -> Vec<u8> { vec![] }
	}
}

mod crypto {
    use super::*;
    use sp_runtime::app_crypto::{app_crypto, key_types::DUMMY};

    mod dummy {
        use crate::tests::sp_core_crypto_dummy::Dummy;
        pub type Public = Dummy;
        pub type Signature = Dummy;
        pub type Pair = Dummy;
    }

    app_crypto!(dummy, DUMMY);

	impl sp_runtime::traits::IdentifyAccount for Public {
		type AccountId = AccountId;

		fn into_account(self) -> Self::AccountId { 11u64 }
	}
}

parameter_types! {
  pub const BlockHashCount: u64 = 250;
  pub const MaximumBlockWeight: Weight = 1024;
  pub const MaximumBlockLength: u32 = 2 * 1024;
  pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for Test {
  type Origin = Origin;
  type Call = Call;
  type Index = u64;
  type BlockNumber = u64;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type AccountId = AccountId;
  type Lookup = IdentityLookup<Self::AccountId>;
  type Header = Header;
  type Event = ();
  type BlockHashCount = BlockHashCount;
  type MaximumBlockWeight = MaximumBlockWeight;
  type MaximumBlockLength = MaximumBlockLength;
  type AvailableBlockRatio = AvailableBlockRatio;
  type Version = ();
  type ModuleToIndex = ();
}

impl timestamp::Trait for Test {
  type Moment = u64;
  type OnTimestampSet = ();
  type MinimumPeriod = ();
}

pub type Extrinsic = TestXt<Call, ()>;
type SubmitPFTransaction = system::offchain::TransactionSubmitter<
	crypto::Public,
    Test,
    Extrinsic
>;

parameter_types! {
  pub const BlockFetchDur: u64 = 1;
}

impl system::offchain::CreateTransaction<Test, Extrinsic> for Test {
	type Public = crypto::Public;
	type Signature = crypto::Signature;

	fn create_transaction<TSigner: system::offchain::Signer<Self::Public, Self::Signature>> (
		call: Call,
		_public: Self::Public,
		account: <Test as system::Trait>::AccountId,
		_index: <Test as system::Trait>::Index,
	) -> Option<(Call, <Extrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
        let extra = ();
		Some((call, (account, extra)))
	}
}

pub type PriceFetchModule = Module<Test>;

impl Trait for Test {
  type Event = ();
  type Call = Call;
  type SubmitSignedTransaction = SubmitPFTransaction;
  type SubmitUnsignedTransaction = SubmitPFTransaction;

  // Wait period between automated fetches. Set to 0 disable this feature.
  //   Then you need to manucally kickoff pricefetch
  type BlockFetchDur = BlockFetchDur;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> runtime_io::TestExternalities {
  system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

#[test]
fn it_works_for_default_value() {
  new_test_ext().execute_with(|| {

    assert_eq!(1, 1);
  });
}
