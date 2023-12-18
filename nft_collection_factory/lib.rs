#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Metadata, PSP34Enumerable, Ownable)]
#[openbrush::contract]
mod nft_collection_factory {
    use common_traits::nft_collection_factory::impls::{
        collectionfactorytrait_external, CollectionFactoryTrait,
    };
    use common_traits::nft_collection_factory::types::{
        CollectionFactoryData, CollectionFactoryError, CollectionInfo,
    };
    use ink::env::CallFlags;
    use ink::ToAccountId;
    use ink::prelude::vec;
    use my_collection::CollectionRef;
    use openbrush::contracts::psp34::Id;
    use openbrush::contracts::traits::psp34::*;
    use openbrush::{
        contracts::{
            ownable, psp34,
            psp34::{
                extensions::{enumerable, metadata},
                PSP34Impl,
            },
        },
        traits::Storage,
    };
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct NftCollectionFactory {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        enumerable: enumerable::Data,
        #[storage_field]
        collection_data: CollectionFactoryData,
    }

    impl CollectionFactoryTrait for NftCollectionFactory {}

    //impl CollectionTrait for NftCollectionFactory {}
    impl NftCollectionFactory {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(collection_code_hash: Hash, creation_fee: Balance) -> Self {
            let mut instance = NftCollectionFactory::default();

            instance
                .initialize(collection_code_hash, creation_fee)
                .ok()
                .unwrap();

            instance
        }

        #[ink(message)]
        pub fn initialize(
            &mut self,
            collection_code_hash: Hash,
            creation_fee: Balance,
        ) -> Result<(), CollectionFactoryError> {
            self.collection_data.collection_code_hash = collection_code_hash;
            self.collection_data.creation_fee = creation_fee;

            Ok(())
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn create_collection(
            &mut self,
            name: String,
            symbol: String,
            base_uri: String,
            mint_to: AccountId,
        ) -> Result<(), CollectionFactoryError> {
            let creator = Self::env().caller();

            let id = self.collection_data.collection_count;

            let collection_contract = CollectionRef::new(creator, name.clone(), symbol)
                .code_hash(self.collection_data.collection_code_hash)
                .endowment(0)
                .salt_bytes(id.to_le_bytes())
                .instantiate();
            let contract_account: AccountId = collection_contract.to_account_id();
            self.collection_data.collection_count += 1;

            // Check if this contract has been approved to be able to transfer the NFT on owner behalf
            let allowance = PSP34Ref::allowance(
                &contract_account,
                creator,
                self.env().account_id(),
                Some(Id::U64(id)),
            );
            if !allowance {
                return Err(CollectionFactoryError::NotApproved);
            }

            // // Transfer Token from Caller to Plat Contract
            if PSP34Ref::transfer_builder(
                &contract_account,
                self.env().account_id(),
                Id::U64(id.clone()),
                Vec::<u8>::new(),
            )
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .invoke()
            .is_err()
            {
                return Err(CollectionFactoryError::CannotTransfer);
            }

            let creator_collections = self.collection_data.creator_collections.get(&creator);
            // Existing collection for creator
            if let Some(mut collections) = creator_collections {
                collections.push(contract_account);
                self.collection_data
                    .creator_collections
                    .insert(&creator, &collections);
            } else {
                // First collection for that creator
                let collections = vec![contract_account];
                self.collection_data
                    .creator_collections
                    .insert(&creator, &collections);
            }
            let new_collection = CollectionInfo {
                name,
                uri: base_uri,
                nft_collection_address: Some(contract_account),
                mint_to: Some(mint_to),
                creator: Some(creator),
            };
            self.collection_data
                .collection_info
                .insert(&contract_account, &new_collection);

            self.collection_data
                .collection_by_id
                .insert(&self.collection_data.collection_count, &contract_account);
            Ok(())
        }

        #[ink(message)]
        pub fn claim_nft(
            &mut self,
            //nft_contract_address: AccountId,
            token_id: Id,
        ) -> Result<(), CollectionFactoryError> {

            
            Ok(())
        }
    }
}
