use {crate::state::*, crate::utils::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(smart_authority_bump: u8)]
pub struct Initialize<'info> {
    initializer: Signer<'info>,
    #[account(
        init,
        seeds = [SMART_AUTHORITY_SEED],
        bump = smart_authority_bump,
        payer = initializer
    )]
    smart_authority: Account<'info, Authority>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, smart_authority_bump: u8) -> ProgramResult {
    ctx.accounts.smart_authority.bump = smart_authority_bump;
    Ok(())
}
