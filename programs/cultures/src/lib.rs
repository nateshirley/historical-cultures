use anchor_lang::prelude::*;

declare_id!("CJLQC725532PSGMMANkSviJvmz7wNRxoar6MDwi8V3Kt");

#[program]
pub mod cultures {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, _culture_bump: u8) -> ProgramResult {
        Ok(())
    }
}

const CULTURE_SEED: &[u8] = b"culture";

#[derive(Accounts)]
#[instruction(culture_bump: u8)]
pub struct Initialize<'info> {
    payer: Signer<'info>,
    #[account(
        init,
        seeds = [CULTURE_SEED],
        bump = culture_bump,
        payer = payer,
    )]
    culture: Account<'info, Culture>,
    system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Culture {
    pub id: u8,
}
