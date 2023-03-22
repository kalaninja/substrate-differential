use codec::{Decode, Encode};
use frame_support::{
    dispatch::DispatchInfo,
    traits::{Get, GetCallMetadata},
    weights::Weight,
};
use scale_info::TypeInfo;
use sp_runtime::{
    traits::{DispatchInfoOf, Dispatchable, SignedExtension},
    transaction_validity::{InvalidTransaction, TransactionValidityError},
};
use sp_std::{marker::PhantomData, prelude::*};

use crate::pallet::ConsumedWeight;
use crate::Config;

/// Extension to check if the pallets weight distribution is respected.
#[derive(Encode, Decode, Clone, Eq, PartialEq, Default, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct DifferentiatePallets<T: Config + Send + Sync>(PhantomData<T>);

impl<T: Config + Send + Sync> DifferentiatePallets<T>
where
    <T as Config>::RuntimeCall: Dispatchable<Info = DispatchInfo>,
{
    /// Create a new instance of the extension.
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// Do the pre-dispatch checks. This can be applied to both signed and unsigned transactions.
    pub fn do_pre_dispatch(
        call: &<T as Config>::RuntimeCall,
        info: &DispatchInfoOf<<T as Config>::RuntimeCall>,
    ) -> Result<(), TransactionValidityError> {
        let pallet_id = call.get_call_metadata().pallet_name;

        // skip if pallet limit is not set
        if let Some(limit) = T::PalletsWeightDistribution::get().get(&pallet_id) {
            let block_weights = T::BlockWeights::get();
            let pallet_max_weight = Weight::from_parts(
                limit.mul_floor(block_weights.max_block.ref_time()),
                limit.mul_floor(block_weights.max_block.proof_size()),
            );
            let extrinsic_weight = info
                .weight
                .saturating_add(block_weights.get(info.class).base_extrinsic);

            ConsumedWeight::<T>::try_mutate_exists(
                pallet_id.as_bytes(),
                |maybe_consumed_weight| {
                    maybe_consumed_weight
                        .unwrap_or(Weight::zero())
                        .checked_add(&extrinsic_weight)
                        .filter(|new_weight| new_weight.all_lte(pallet_max_weight))
                        .map(|new_weight| *maybe_consumed_weight = Some(new_weight))
                        .ok_or(InvalidTransaction::ExhaustsResources)
                },
            )?;
        }

        Ok(())
    }
}

impl<T: Config + Send + Sync> SignedExtension for DifferentiatePallets<T>
where
    <T as Config>::RuntimeCall: Dispatchable<Info = DispatchInfo>,
{
    const IDENTIFIER: &'static str = "DifferentiatePallets";
    type AccountId = T::AccountId;
    type Call = <T as Config>::RuntimeCall;
    type AdditionalSigned = ();
    type Pre = ();

    fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
        Ok(())
    }

    fn pre_dispatch(
        self,
        _who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        Self::do_pre_dispatch(call, info)
    }

    fn pre_dispatch_unsigned(
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> Result<(), TransactionValidityError> {
        Self::do_pre_dispatch(call, info)
    }
}

impl<T: Config + Send + Sync> sp_std::fmt::Debug for DifferentiatePallets<T> {
    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        write!(f, "DifferentiatePallets")
    }

    #[cfg(not(feature = "std"))]
    fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        Ok(())
    }
}
