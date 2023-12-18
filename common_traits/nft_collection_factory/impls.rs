use ink::prelude::vec::Vec;

use openbrush::{
    modifiers,
    traits::{AccountId, Storage, String, Hash},
};

use openbrush::contracts::{
    ownable,
    ownable::only_owner,
    psp34,
    psp34::{
        extensions::metadata::{Id, PSP34MetadataImpl},
        PSP34Error, PSP34Impl,
    },
};

use crate::nft_collection_factory::types::{CollectionFactoryData, CollectionFactoryError, CollectionInfo};
use crate::collection::types::CollectionError;
use ink::env::CallFlags;
use openbrush::contracts::traits::psp34::*;

#[openbrush::trait_definition]
pub trait CollectionFactoryTrait:
    Storage<CollectionFactoryData>
    + Storage<ownable::Data>
    + PSP34Impl
    + PSP34MetadataImpl
    + psp34::extensions::metadata::Internal
{
    #[ink(message)]
    fn get_collections_address_by_creator(
        &self,
        creator: AccountId,
    ) -> Result<Vec<AccountId>, CollectionFactoryError> {
        let collections_by_creator = self.data::<CollectionFactoryData>().creator_collections.get(&creator).unwrap_or_default();

        Ok(collections_by_creator)
    }

    #[ink(message)]
    fn get_current_id(
        &self
    ) -> Result<u64, CollectionFactoryError> {
        let id = self.data::<CollectionFactoryData>().collection_count;

        Ok(id)
    }

    #[ink(message)]
    fn get_collection_by_nft_address(
        &self, nft_contract_address: AccountId
    ) -> Option<CollectionInfo> {
        self.data::<CollectionFactoryData>().collection_info.get(&nft_contract_address)
    }
    
    #[ink(message)]
    fn get_contract_by_id(&self, id: u64) -> Option<AccountId> {
        self.data::<CollectionFactoryData>().collection_by_id.get(&id)
    }
    
    #[ink(message)]
    fn set_collection_hash(&mut self, collection_hash: Hash) -> Result<(),CollectionFactoryError> {
        self.data::<CollectionFactoryData>().collection_code_hash = collection_hash;
        Ok(())
    }

}
