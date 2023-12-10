use openbrush::contracts::psp34::PSP34Error;
use openbrush::traits::String;
#[openbrush::wrapper]
pub type CollectionRef = dyn CollectionTraitRef;

/// Collection method definitions.
/// Actually only methods used by other contract (cross-contract call) are needed.
#[openbrush::trait_definition]
pub trait CollectionTraitRef {

    #[ink(message)]
    fn mint_collection(&mut self, id: u32, name: String, collection_type: String) -> Result<(), PSP34Error>;
}
