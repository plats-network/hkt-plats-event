use ink::prelude::vec::Vec;
use openbrush::traits::{AccountId, String};

use crate::nft_collection_factory::types::CollectionFactoryError;
use openbrush::{
    traits::{
        Balance,
    },
    contracts::{
        traits::psp34::{
            Id
        },
        traits::psp22::*,
        traits::psp34::*
    },
};

#[openbrush::wrapper]
pub type CollectionFactoryRef = dyn CollectionFactoryTraitRef;


#[openbrush::wrapper]
pub type Psp34Ref = dyn PSP34;

/// Collection method definitions.
/// Actually only methods used by other contract (cross-contract call) are needed.
#[openbrush::trait_definition]
pub trait CollectionFactoryTraitRef {
    #[ink(message)]
    fn get_collections_by_creator(
        &self,
        creator: AccountId,
    ) -> Result<Vec<AccountId>, CollectionFactoryError>;
}
