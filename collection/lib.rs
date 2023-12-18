#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::my_collection::{Collection, CollectionRef};

#[openbrush::implementation(PSP34, PSP34Metadata, PSP34Enumerable, Ownable)]
#[openbrush::contract]
pub mod my_collection {
    use common_traits::collection::impls::CollectionTrait;
    use common_traits::collection::traits::collectiontrait_external;
    use openbrush::{
        contracts::ownable::*,
        contracts::psp34::extensions::{burnable::*, enumerable::*, metadata::*},
        traits::Storage,
    };
   //use openbrush::
    
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Collection {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        enumerable: enumerable::Data,
    }

    impl CollectionTrait for Collection {}

    impl Collection {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, name: String, symbol: String) -> Self {
            let mut instance = Self::default();
            openbrush::contracts::ownable::Internal::_init_with_owner(&mut instance, contract_owner);
            instance._set_attribute(
                Id::U8(0),
                String::from("name"),
                name,
            );
            instance._set_attribute(
                Id::U8(0),
                String::from("symbol"),
                symbol,
            );
            instance
        }
    }
}
