use ink::prelude::vec::Vec;

use openbrush::{
    modifiers,
    traits::{
        AccountId,
        Storage,
        String,
    },
};

use openbrush::contracts::{
    ownable,
    ownable::only_owner,
    psp34,
    psp34::{
        extensions::metadata::{
            Id,
            PSP34MetadataImpl,
        },
        PSP34Error,
        PSP34Impl,
    },
};
use crate::collection::types::CollectionError;


#[openbrush::trait_definition]
pub trait CollectionTrait:
    Storage<psp34::Data>
    + Storage<ownable::Data>
    // + Storage<metadata::Data>
    + PSP34Impl
    + PSP34MetadataImpl
    + psp34::extensions::metadata::Internal
{
    
    #[ink(message)]
    fn mint_collection(&mut self, id: u32, name: String, collection_type: String) -> Result<u32, CollectionError> {
        let caller = Self::env().caller();
        
        self._mint_to(caller, Id::U32(id)).map_err(|_| CollectionError::CannotMint)?;

        self._set_attribute(Id::U32(id), "name".into(), name);
        self._set_attribute(Id::U32(id), "type".into(), collection_type);

        Ok(id)
    }
}