use std::convert::TryFrom;
use {crate::state::*, crate::utils::*, anchor_lang::prelude::*, anchor_spl::token};

#[derive(Accounts)]
#[instruction(membership_bump: u8, creator_stake_pool_bump: u8)]
pub struct ChangeCreatorStake<'info> {
    member: Signer<'info>,
    culture: Account<'info, Culture>,
    #[account(
        mut,
        seeds = [MEMBERSHIP_SEED, culture.key().as_ref(), member.key().as_ref()],
        bump = membership_bump,
        constraint = membership.authority == member.key()
    )]
    membership: Account<'info, Membership>,
    #[account(
        mut,
        constraint = creator_token_account.owner == member.key(),
        constraint = creator_token_account.mint == culture.creator_mint,
    )]
    creator_token_account: Account<'info, token::TokenAccount>,
    #[account(
        mut,
        constraint = creator_stake_pool.key() == find_creator_stake_pool(culture.key(), creator_stake_pool_bump)
    )]
    creator_stake_pool: Account<'info, token::TokenAccount>,
    #[account(
        seeds = [STAKE_AUTHORITY_SEED],
        bump = stake_authority.bump,
    )]
    stake_authority: Account<'info, Authority>,
    token_program: Program<'info, token::Token>,
    system_program: Program<'info, System>,
}
pub fn handler(
    ctx: Context<ChangeCreatorStake>,
    _membership_bump: u8,
    _creator_stake_pool_bump: u8,
    amount: i64,
) -> ProgramResult {
    //transfer to/from the stake pool
    let new_stake: u64;
    if amount > 0 {
        let unsigned_amount = u64::try_from(amount).unwrap();
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.creator_token_account.to_account_info(),
                    to: ctx.accounts.creator_stake_pool.to_account_info(),
                    authority: ctx.accounts.member.to_account_info(),
                },
            ),
            unsigned_amount,
        )?;
        new_stake = ctx
            .accounts
            .membership
            .creator_stake
            .checked_add(unsigned_amount)
            .unwrap();
    } else {
        let unsigned_amount = u64::try_from(amount.checked_abs().unwrap()).unwrap();
        if unsigned_amount <= ctx.accounts.membership.creator_stake {
            anchor_spl::token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    anchor_spl::token::Transfer {
                        from: ctx.accounts.creator_stake_pool.to_account_info(),
                        to: ctx.accounts.creator_token_account.to_account_info(),
                        authority: ctx.accounts.stake_authority.to_account_info(),
                    },
                )
                .with_signer(&[&[STAKE_AUTHORITY_SEED, &[ctx.accounts.stake_authority.bump]]]),
                unsigned_amount,
            )?;
            new_stake = ctx
                .accounts
                .membership
                .creator_stake
                .checked_sub(unsigned_amount)
                .unwrap();
        } else {
            return Err(ErrorCode::InsufficientStakeWithdraw.into());
        }
    }
    //reflect changes in membership account
    ctx.accounts.membership.creator_stake = new_stake;
    //if it's a symmetrical culture, edit audience stake as well
    if ctx.accounts.culture.creator_mint == ctx.accounts.culture.audience_mint {
        ctx.accounts.membership.audience_stake = new_stake;
    }
    Ok(())
}
