use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Culture {
    pub name: String,
    pub collection: Pubkey,
    pub treasury: Pubkey,
    pub creator_mint: Pubkey,
    pub creator_count: u32,
    pub audience_mint: Pubkey,
    pub audience_count: u32,
    pub bump: u8,
}
