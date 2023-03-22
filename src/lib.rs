//! # Substrate Differential Pallet
//!
//! This pallet allows to configure the distribution of the block weight between the pallets. The
//! distribution is configured by the `PalletsWeightDistribution` type, which is a map from the
//! pallet name to the weight percentage.
//!
//! This pallet defines the [`DefferentiatePallets`] extension, which checks the weight consumed by
//! each pallet and compares it to the configured weight distribution. If the consumed weight is
//! greater than the configured weight, the transaction is rejected.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::*, traits::GetCallMetadata};
use frame_system::pallet_prelude::*;
use sp_runtime::traits::Dispatchable;
use sp_std::prelude::*;

pub use differentiate_pallets::DifferentiatePallets;
pub use pallet::*;

use crate::limits::PalletsWeightDistribution;

mod differentiate_pallets;
pub mod limits;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching call type.
        type RuntimeCall: Parameter
            + Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>
            + GetCallMetadata
            + From<frame_system::Call<Self>>;

        /// Pallets weight distribution.
        #[pallet::constant]
        type PalletsWeightDistribution: Get<PalletsWeightDistribution>;
    }

    /// The weight consumed by each pallet in the current block.
    // TODO: Change Key to pallet index when https://github.com/paritytech/substrate/issues/13511 is resolved.
    #[pallet::storage]
    #[pallet::getter(fn consumed_weight)]
    pub(super) type ConsumedWeight<T: Config> = StorageMap<_, Twox64Concat, Vec<u8>, Weight>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(_: T::BlockNumber) {
            let _ = ConsumedWeight::<T>::clear(u32::MAX, None);
        }
    }
}
