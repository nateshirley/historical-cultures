use {crate::state::*, crate::utils::*, anchor_lang::prelude::*, anchor_spl::token};

#[derive(Accounts)]
#[instruction(creator_stake_pool_bump: u8, audience_stake_pool_bump: u8)]
pub struct MintPost<'info> {
    culture: Account<'info, Culture>,
    poster: Signer<'info>,
    #[account(
        constraint = post.culture == culture.key(),
        constraint = post.poster == poster.key(),
    )]
    post: Account<'info, Post>,
    #[account(
        constraint = membership.culture == culture.key(),
        constraint = membership.member == poster.key()
    )]
    membership: Account<'info, Membership>,
    #[account(
        mut,
        address = find_creator_stake_pool(culture.key(), creator_stake_pool_bump)
    )]
    creator_stake_pool: Account<'info, token::TokenAccount>,
    #[account(
        mut,
        address = find_audience_stake_pool(culture.key(), audience_stake_pool_bump)
    )]
    audience_stake_pool: Account<'info, token::TokenAccount>,
}

pub fn handler(ctx: Context<MintPost>) -> ProgramResult {
    let audience_count: u32 = ctx.accounts.culture.audience_count;
    let tokens_staked: u64 = if ctx.accounts.culture.is_symmetrical() {
        ctx.accounts.creator_stake_pool.amount
    } else {
        ctx.accounts.audience_stake_pool.amount
    };
    if ctx.accounts.post.score > minimum_score_to_mint(audience_count, tokens_staked) {
        msg!("minting");
    } else {
        panic!();
    }
    Ok(())
}
