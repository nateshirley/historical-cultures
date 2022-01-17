use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use spl_token_metadata::state::Creator;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct DataV2 {
    /// The name of the asset
    pub name: String,
    /// The symbol for the asset
    pub symbol: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    /// Array of creators, optional
    pub creators: Option<Vec<Creator>>,
    /// Collection
    pub collection: Option<Collection>,
    /// Uses
    pub uses: Option<Uses>,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Uses {
    // 17 bytes + Option byte
    pub use_method: UseMethod, //1
    pub remaining: u64,        //8
    pub total: u64,            //8
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum TokenStandard {
    NonFungible,        // This is a master edition
    FungibleAsset,      // A token with metadata that can also have attrributes
    Fungible,           // A token with simple metadata
    NonFungibleEdition, // This is a limited edition
}

#[derive(Clone)]
pub struct TokenMetadata;
impl anchor_lang::AccountDeserialize for TokenMetadata {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        TokenMetadata::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Ok(TokenMetadata)
    }
}
impl anchor_lang::Id for TokenMetadata {
    fn id() -> Pubkey {
        spl_token_metadata::id()
    }
}
