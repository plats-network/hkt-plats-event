
use openbrush::contracts::psp34::Id;
use openbrush::traits::{
    AccountId,
    Balance,
    String,
};



#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CollectionError {
    CannotInitialize,
    CannotMint,
    NotOwner,
    NotExists,
}