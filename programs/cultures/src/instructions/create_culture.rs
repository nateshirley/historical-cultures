use {crate::state::*, crate::utils::*, anchor_lang::prelude::*, anchor_spl::token};

#[derive(Accounts)]
#[instruction(culture_bump: u8, name: String)]
pub struct CreateCulture<'info> {
    payer: Signer<'info>,
    #[account(
        init,
        seeds = [CULTURE_SEED, name.clone().to_seed_format().as_bytes()],
        bump = culture_bump,
        payer = payer,
        space = 161, //could also do this custom if i wanted based on length of the string. prob not worth it
    )]
    culture: Account<'info, Culture>,
    collection: UncheckedAccount<'info>,
    creator_mint: Box<Account<'info, token::Mint>>,
    #[account(
        init,
        seeds = [C_STAKE_SEED, culture.key().as_ref()],
        bump,
        payer = payer,
        token::mint = creator_mint,
        token::authority = stake_authority,
    )]
    creator_stake_pool: Box<Account<'info, token::TokenAccount>>,
    #[account(
        init,
        seeds = [C_REDEMPTION_SEED, culture.key().as_ref()],
        bump,
        payer = payer,
        mint::decimals = 0,
        mint::authority = stake_authority,
        mint::freeze_authority = stake_authority,
    )]
    creator_redemption_mint: Box<Account<'info, token::Mint>>,
    audience_mint: Account<'info, token::Mint>,
    #[account(
        init,
        seeds = [A_STAKE_SEED, culture.key().as_ref()],
        bump,
        payer = payer,
        token::mint = creator_mint,
        token::authority = stake_authority,
    )]
    audience_stake_pool: Account<'info, token::TokenAccount>,
    #[account(
        init,
        seeds = [A_REDEMPTION_SEED, culture.key().as_ref()],
        bump,
        payer = payer,
        mint::decimals = 0,
        mint::authority = stake_authority,
        mint::freeze_authority = stake_authority,
    )]
    audience_redemption_mint: Account<'info, token::Mint>,
    #[account(
        seeds = [STAKE_AUTHORITY_SEED],
        bump = stake_authority.bump
    )]
    stake_authority: Account<'info, Authority>,
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, token::Token>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateCulture>, _culture_bump: u8, name: String) -> ProgramResult {
    ctx.accounts.culture.name = name.to_seed_format();
    ctx.accounts.culture.collection = ctx.accounts.collection.key();
    ctx.accounts.culture.creator_mint = ctx.accounts.creator_mint.key();
    ctx.accounts.culture.audience_mint = ctx.accounts.audience_mint.key();
    // ctx.accounts.culture.stake_pool = ctx.accounts.stake_pool.key();
    // ctx.accounts.culture.redemption_mint = ctx.accounts.redemption_mint.key();
    //leave collection for now and come back to it
    //but should probably do it in a separate, collection factory program
    //i'll do all the staking/posts/voting first
    Ok(())
}
