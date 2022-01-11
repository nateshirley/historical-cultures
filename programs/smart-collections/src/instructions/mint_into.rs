use anchor_spl::{associated_token, token};
use anchor_spl_token_metadata::anchor_token_metadata;
use {crate::state::*, crate::utils::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct MintInto<'info> {
    #[account(
        mut,
        constraint = smart_collection.mint_authority.unwrap() == collection_mint_authority.key(),
        constraint = smart_collection.has_remaining_supply()
    )]
    smart_collection: Account<'info, SmartCollection>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = smart_authority,
    )]
    item_mint: Account<'info, token::Mint>, //must also be signer,
    payer: Signer<'info>,
    receiver: AccountInfo<'info>,
    #[account(
        init,
        payer = payer,
        associated_token::authority = receiver,
        associated_token::mint = item_mint,
    )]
    receiver_token_account: Account<'info, token::TokenAccount>,
    collection_mint_authority: Signer<'info>,
    #[account(
        seeds = [SMART_AUTHORITY_SEED],
        bump = smart_authority.bump
    )]
    smart_authority: Account<'info, Authority>,
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, token::Token>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<MintInto>) -> ProgramResult {
    let seeds = &[
        &SMART_AUTHORITY_SEED[..],
        &[ctx.accounts.smart_authority.bump],
    ];
    token::mint_to(
        ctx.accounts
            .into_mint_item_to_receiver_context()
            .with_signer(&[seeds]),
        1,
    )?;

    ctx.accounts.smart_collection.supply =
        ctx.accounts.smart_collection.supply.checked_add(1).unwrap();
    //mint one token to the receiver
    //create metadata for the mint
    //create master edition
    Ok(())
}

impl<'info> MintInto<'info> {
    fn into_mint_item_to_receiver_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::MintTo {
            mint: self.item_mint.to_account_info(),
            to: self.receiver_token_account.to_account_info(),
            authority: self.smart_authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
