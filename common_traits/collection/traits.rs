use openbrush::contracts::psp34::extensions::{enumerable::*, metadata::*};
use crate::collection::types::CollectionError;
use openbrush::traits::String;

#[openbrush::wrapper]
pub type CollectionRef = dyn CollectionTrait + PSP34 + PSP34Metadata;

/// Collection method definitions.
/// Actually only methods used by other contract (cross-contract call) are needed.
#[openbrush::trait_definition]
pub trait CollectionTrait{
    #[ink(message)]
    fn set_base_uri(&mut self, uri: String) -> Result<(), CollectionError>;
}
