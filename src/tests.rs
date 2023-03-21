use frame_support::{
    assert_err, assert_ok, dispatch::DispatchInfo, traits::GetCallMetadata, weights::Weight,
};
use sp_runtime::{transaction_validity::InvalidTransaction, Perbill};

use crate::differentiate_pallets::DifferentiatePallets;
use crate::limits::{PalletWeightDistributionError, PalletsWeightDistribution};
use crate::mock::*;
use crate::pallet::ConsumedWeight;

#[test]
fn can_configure_pallets() {
    let system_weight = Perbill::from_percent(50);
    let pallets = PalletsWeightDistribution::build_with::<PalletInfo>()
        .add::<System>(system_weight)
        .add::<Differential>(Perbill::from_percent(50))
        .build_or_panic();

    assert_eq!(pallets.pallets.len(), 2);
    assert_eq!(pallets.get("System"), Some(&system_weight));
    assert_eq!(pallets.get("Unknown"), None);
}

#[test]
fn cannot_configure_same_pallet_twice() {
    let pallets = PalletsWeightDistribution::build_with::<PalletInfo>()
        .add::<System>(Perbill::from_percent(50))
        .add::<System>(Perbill::from_percent(50));

    assert!(matches!(
        pallets.build(),
        Err(PalletWeightDistributionError::PalletAlreadyExists)
    ));
}

#[test]
fn cannot_configure_pallets_with_total_weight_exceeded() {
    let pallets = PalletsWeightDistribution::build_with::<PalletInfo>()
        .add::<System>(Perbill::from_percent(50))
        .add::<Differential>(Perbill::from_percent(51));

    assert!(matches!(
        pallets.build(),
        Err(PalletWeightDistributionError::TotalWeightExceeded)
    ));
}

#[test]
fn pallet_cannot_exceed_limit() {
    new_test_ext().execute_with(|| {
        let info = DispatchInfo {
            weight: RuntimeBlockWeights::get().max_block,
            ..Default::default()
        };

        assert_err!(
            DifferentiatePallets::<Test>::do_pre_dispatch(CALL_LIMITED, &info),
            InvalidTransaction::ExhaustsResources
        );
    });
}

#[test]
fn pallet_can_pass_check() {
    new_test_ext().execute_with(|| {
        let weight = Weight::from_parts(1, 1);
        let info = DispatchInfo {
            weight,
            ..Default::default()
        };

        assert_ok!(DifferentiatePallets::<Test>::do_pre_dispatch(
            CALL_LIMITED,
            &info
        ));

        assert!(ConsumedWeight::<Test>::get(
            CALL_LIMITED.get_call_metadata().pallet_name.as_bytes()
        )
        .unwrap()
        .all_gte(weight));

        assert_ok!(DifferentiatePallets::<Test>::do_pre_dispatch(
            CALL_LIMITED,
            &info
        ));
    });
}

#[test]
fn pallet_unlimited() {
    new_test_ext().execute_with(|| {
        let info = DispatchInfo {
            weight: RuntimeBlockWeights::get().max_block,
            ..Default::default()
        };

        assert_ok!(DifferentiatePallets::<Test>::do_pre_dispatch(
            CALL_UNLIMITED,
            &info
        ));
    });
}
