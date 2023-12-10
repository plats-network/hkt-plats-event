#![cfg_attr(not(feature = "std"), no_std, no_main)]


pub use self::my_collection::{CollectionRef, Collection};

#[openbrush::implementation(PSP34, PSP34Metadata, PSP34Enumerable, Ownable)]
#[openbrush::contract]
pub mod my_collection {
    use openbrush::{
        contracts::{
            ownable,
            psp34,
            psp34::{
                extensions::{
                    enumerable,
                    metadata,
                },
                PSP34Impl,
            },
            // reentrancy_guard,
        },
        traits::Storage,
    };

    // upgradable
    use common_traits::upgradable::UpgradableTrait;
    use common_traits::collection::impls::{collectiontrait_external, CollectionTrait};
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
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
    impl CollectionTrait for Collection{}
    impl Collection {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self::default();

            instance
        }
    }
}
