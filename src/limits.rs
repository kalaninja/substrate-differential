//! Pallets limits configuration structures.
//!
//! `substrate_differential` is a pallet that allows to configure the distribution of the block
//! weight between the pallets. This module contains the structures that are used to configure the
//! pallets distribution, which should be passed to `substrate_differential` configuration when
//! runtime is being built.

use frame_support::traits::PalletInfo;
use scale_info::TypeInfo;
use sp_runtime::{traits::CheckedAdd, Perbill, RuntimeDebug};
use sp_std::{collections::btree_map::BTreeMap, marker::PhantomData, prelude::*};

#[derive(Debug, Eq, PartialEq)]
pub enum PalletWeightDistributionError {
    PalletAlreadyExists,
    TotalWeightExceeded,
}

/// Pallets weight distribution configuration.
///
/// The distribution is a map from the pallet name to the weight percentage. The sum of the
/// percentages should not exceed 100%. The percentages are represented as `Perbill` values.
///
/// The weight for a pallet cannot be configured more than once.
#[derive(RuntimeDebug, Clone, codec::Encode, TypeInfo)]
pub struct PalletsWeightDistribution {
    pub(crate) pallets: BTreeMap<&'static str, Perbill>,
}

impl Default for PalletsWeightDistribution {
    fn default() -> Self {
        Self {
            pallets: Default::default(),
        }
    }
}

impl PalletsWeightDistribution {
    /// Start building the pallets weight distribution.
    pub fn build_with<T: PalletInfo>() -> PalletsWeightDistributionBuilder<T> {
        PalletsWeightDistributionBuilder {
            pallets: Vec::new(),
            _pallet_info: PhantomData,
        }
    }

    /// Get the weight fraction for the given pallet.
    pub fn get(&self, pallet: &'static str) -> Option<&Perbill> {
        self.pallets.get(pallet)
    }
}

struct PalletWeightFraction {
    pallet_id: &'static str,
    fraction: Perbill,
}

/// Builder for the pallets weight distribution.
pub struct PalletsWeightDistributionBuilder<T: PalletInfo> {
    pallets: Vec<PalletWeightFraction>,
    _pallet_info: PhantomData<T>,
}

impl<T: PalletInfo> PalletsWeightDistributionBuilder<T> {
    /// Add a pallet to the distribution with the given weight fraction.
    pub fn add<P: 'static>(mut self, fraction: Perbill) -> PalletsWeightDistributionBuilder<T> {
        let pallet_id = T::name::<P>().expect("Pallet is not part of the runtime");

        self.pallets.push(PalletWeightFraction {
            pallet_id,
            fraction,
        });

        self
    }

    /// Construct the pallets weight distribution.
    pub fn build(self) -> Result<PalletsWeightDistribution, PalletWeightDistributionError> {
        self.pallets
            .iter()
            .map(|x| x.fraction)
            .try_fold(Perbill::zero(), |acc, x| acc.checked_add(&x))
            .ok_or(PalletWeightDistributionError::TotalWeightExceeded)?;

        let pallets = self
            .pallets
            .into_iter()
            .try_fold(BTreeMap::new(), |mut acc, x| {
                match acc.insert(x.pallet_id, x.fraction) {
                    None => Some(acc),
                    Some(_) => None,
                }
            })
            .ok_or(PalletWeightDistributionError::PalletAlreadyExists)?;

        Ok(PalletsWeightDistribution { pallets })
    }

    /// Construct the pallets weight distribution, panicking if it fails.
    ///
    /// This is a convenience method for calling whenever a runtime is being constructed.
    pub fn build_or_panic(self) -> PalletsWeightDistribution {
        self.build()
            .unwrap_or_else(|e| panic!("Failed to build pallets weight distribution: {:?}", e))
    }
}
