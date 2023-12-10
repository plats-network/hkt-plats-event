use ink::prelude::vec::Vec;

use openbrush::{
    modifiers,
    traits::{AccountId, Storage, String},
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
        let collections_by_creator = self.data::<CollectionFactoryData>().creator_collection.get(&creator).unwrap_or_default();

        Ok(collections_by_creator)
    }

    #[ink(message)]
    fn get_current_id(
        &self
    ) -> Result<u32, CollectionFactoryError> {
        let id = self.data::<CollectionFactoryData>().current_id;

        Ok(id)
    }


    #[ink(message)]
    fn get_all_collection_info_by_creator(
        &self,
        creator: AccountId,
    ) -> Result<Vec<CollectionInfo>, CollectionFactoryError> {
        let mut collections = Vec::new();
        // get contract collection from creator
        let collections_by_creator = self.data::<CollectionFactoryData>().creator_collection.get(&creator).unwrap_or_default();
        for collection in collections_by_creator{
            let collection_info = self.data::<CollectionFactoryData>().collection_info.get(&collection).unwrap_or_default();
            collections.push(collection_info);
        }
        
        Ok(collections)
    }

    #[ink(message)]
    fn mint_collection(&mut self, name: String, collection_type: String, base_uri: String) -> Result<(), CollectionFactoryError> {
        let caller = Self::env().caller();
        let id = self.data::<CollectionFactoryData>().current_id;
        self._set_attribute(Id::U32(id), "name".into(), name.clone());
        self._set_attribute(Id::U32(id), "type".into(), collection_type.clone());
        self._set_attribute(Id::U32(id), "uri".into(), base_uri.clone());
        self._mint_to(caller, Id::U32(id)).map_err(|_| CollectionFactoryError::CannotMint)?;


        self.data::<CollectionFactoryData>().current_id = id + 1;

        Ok(())
    }

}
