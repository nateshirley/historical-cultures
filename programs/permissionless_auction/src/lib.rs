use anchor_lang::prelude::*;
use cultures::Culture;
declare_id!("83PHTTiss3tkzVzEUgYWF7CgYEyg2729o3LXKkatkiX8");
const CULTURE_SEED: &[u8] = b"culture";

#[program]
pub mod permissionless_auction {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, _culture_bump: u8) -> ProgramResult {
        Ok(())
    }
}

pub fn cultures_address(seeds_with_nonce: &[&[u8]]) -> Pubkey {
    Pubkey::create_program_address(seeds_with_nonce, &cultures::id()).unwrap()
}

#[derive(Accounts)]
#[instruction(culture_bump: u8)]
pub struct Initialize<'info> {
    #[account(
        address = cultures_address(&[CULTURE_SEED, &[culture_bump]])
    )]
    culture: Account<'info, Culture>,
}
