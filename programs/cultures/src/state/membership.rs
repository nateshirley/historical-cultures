use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Membership {
    pub culture: Pubkey,
    pub authority: Pubkey,
    pub creator_stake: u64,
    pub audience_stake: u64,
    pub bump: u8,
}
//8 + str + (32 * 4) + 4 + 1
// = 141 + str
//str = 20 (16chars + 4 setup)
