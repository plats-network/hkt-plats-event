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
        pub fn new() -> Self {
            let instance = NftCollectionFactory::default();


            instance
        }

        #[ink(message)]
        pub fn create_collection(
            &mut self,
            //collection_code_hash: Hash,
            name: String,
            collection_type: String,
            base_uri: String,
            record_id: u32,
        ) -> Result<(), CollectionFactoryError> {
            let creator = Self::env().caller();

            let collection_info = CollectionInfo {
                creator: Some(creator),
                record_id,
            };
            let id = self.collection_data.current_id;

            // let collection_contract = CollectionRef::new()
            //     .code_hash(collection_code_hash)
            //     .endowment(0)
            //     .salt_bytes(id.to_le_bytes())
            //     .instantiate();
            // let contract_account: AccountId = collection_contract.to_account_id();
            self.mint_collection(name, collection_type, base_uri)?;
            // self.collection_data
            //     .collection_info
            //     .insert(&contract_account, &collection_info);
            // self.collection_data
            //     .record_id_collection
            //     .insert(&record_id, &contract_account);
            // let mut collections_of_caller = self
            //     .collection_data
            //     .creator_collection
            //     .get(&creator)
            //     .unwrap_or_default();
            // collections_of_caller.push(contract_account);
            // self.collection_data
            //     .creator_collection
            //     .insert(&creator, &collections_of_caller);

            // Check if this contract has been approved to be able to transfer the NFT on owner behalf
            let allowance = PSP34Ref::allowance(
                &self.env().account_id(),
                creator,
                self.env().account_id(),
                Some(Id::U32(id)),
            );
            if !allowance {
                return Err(CollectionFactoryError::NotApproved);
            }

            // Transfer Token from Caller to Marketplace Contract
            if PSP34Ref::transfer_builder(
                &self.env().account_id(),
                self.env().account_id(),
                Id::U32(id.clone()),
                Vec::<u8>::new(),
            )
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .invoke()
            .is_err()
            {
                return Err(CollectionFactoryError::CannotTransfer);
            }

            Ok(())
        }

        #[ink(message)]
        pub fn claim_nft(
            &mut self,
            //nft_contract_address: AccountId,
            token_id: Id,
            receiver: AccountId,
        ) -> Result<(), CollectionFactoryError> {
            if PSP34Ref::transfer_builder(
                &self.env().account_id(),
                receiver,
                token_id.clone(),
                Vec::<u8>::new(),
            )
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .invoke()
            .is_err()
            {
                return Err(CollectionFactoryError::ClaimNFTError);
            }

            Ok(())
        }
    }
}
