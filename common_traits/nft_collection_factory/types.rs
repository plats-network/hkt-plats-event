use openbrush::contracts::psp34::Id;
use openbrush::traits::{AccountId, Balance, String};
use ink::prelude::vec::Vec;

#[cfg(feature = "std")]
use ink::storage::traits::StorageLayout;
use openbrush::storage::Mapping;

#[derive(Default, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct CollectionInfo {
    pub record_id: u32,
    pub creator: Option<AccountId>,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct CreatorInfo {
    pub address: AccountId,
    pub claimed: u8,
}

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct CollectionFactoryData {
    pub collection_address: Option<AccountId>,
    /// Collection Info : Collection Contract => CollectionInfo
    pub collection_info: Mapping<AccountId, CollectionInfo>,
    pub record_id_collection: Mapping<u32, AccountId>,
    pub creator_collection: Mapping<AccountId, Vec<AccountId>>,
    pub current_id: u32,
}

/// The Adventure error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CollectionFactoryError {
    NotExists,
    CannotMint,
    ClaimNFTError,
    NotApproved,
    CannotTransfer
}
