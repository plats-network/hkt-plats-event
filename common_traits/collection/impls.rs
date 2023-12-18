use openbrush::traits::{Storage, String};

use crate::collection::types::CollectionError;
use openbrush::contracts::psp34;
use openbrush::{
    contracts::ownable::*,
    contracts::psp34::extensions::{enumerable::*, metadata::*},
};
#[openbrush::trait_definition]
pub trait CollectionTrait:
    Storage<psp34::Data>
    + Storage<ownable::Data>
    + Storage<metadata::Data>
    + psp34::Internal
    + psp34::extensions::metadata::Internal
    + Storage<psp34::extensions::enumerable::Data>
    + PSP34
{
    #[ink(message)]
    fn set_base_uri(&mut self, uri: String) -> Result<(), CollectionError> {
        Ok(())
    }
}
