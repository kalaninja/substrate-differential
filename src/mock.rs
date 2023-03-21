use frame_support::weights::constants::WEIGHT_REF_TIME_PER_SECOND;
use frame_support::{
    parameter_types,
    traits::{ConstU16, ConstU64},
    weights::Weight,
};
use sp_core::{ConstU32, H256};
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

use crate as pallet_differential;
use crate::limits::PalletsWeightDistribution;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Balances: pallet_balances,
        Differential: pallet_differential,
    }
);

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
    pub RuntimeBlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::with_sensible_defaults(
            Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
            NORMAL_DISPATCH_RATIO,
        );
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = RuntimeBlockWeights;
    type BlockLength = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub RuntimePalletsWeightDistribution: PalletsWeightDistribution =
        PalletsWeightDistribution::build_with::<PalletInfo>()
            .add::<System>(Perbill::from_percent(50))
            .build_or_panic();
}

impl pallet_differential::Config for Test {
    type RuntimeCall = RuntimeCall;
    type PalletsWeightDistribution = RuntimePalletsWeightDistribution;
}

pub const CALL_LIMITED: &<Test as frame_system::Config>::RuntimeCall =
    &RuntimeCall::System(frame_system::Call::set_heap_pages { pages: 0u64 });

pub const CALL_UNLIMITED: &<Test as frame_system::Config>::RuntimeCall =
    &RuntimeCall::Balances(pallet_balances::Call::transfer { dest: 0, value: 0 });

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}
