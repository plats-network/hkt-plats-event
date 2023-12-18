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
    //+ Storage<ownable::Data>
    // + Storage<metadata::Data>
    //+ PSP34Impl
    //+ PSP34MetadataImpl
    //+ psp34::extensions::metadata::Internal
{
    #[ink(message)]
    fn set_base_uri(&mut self, uri: String) -> Result<(), CollectionError>{
        Ok(())
    
    }

}