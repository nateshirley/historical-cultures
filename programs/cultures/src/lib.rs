use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use std::convert::TryFrom;
declare_id!("5prqgWu5iSVS8DvYruAFFCNQjAbg1RvojMCRL6dPSPye");

mod instructions;
pub mod state;
pub mod utils;

use instructions::*;
use state::*;

#[program]
pub mod cultures {
    use super::*;

    pub fn initialize_program(
        ctx: Context<InitializeProgram>,
        stake_authority_bump: u8,
    ) -> ProgramResult {
        initialize_program::handler(ctx, stake_authority_bump)
    }

    pub fn create_culture(
        ctx: Context<CreateCulture>,
        culture_bump: u8,
        name: String,
    ) -> ProgramResult {
        create_culture::handler(ctx, culture_bump, name)
    }

    pub fn create_membership(ctx: Context<CreateMembership>, membership_bump: u8) -> ProgramResult {
        //config membership
        create_membership::handler(ctx, membership_bump)
    }

    //if you have a symmetrical culture, use creator stake for changing both posting & voting
    pub fn change_creator_stake(
        ctx: Context<ChangeCreatorStake>,
        membership_bump: u8,
        creator_stake_pool_bump: u8,
        amount: i64,
    ) -> ProgramResult {
        change_creator_stake::handler(ctx, membership_bump, creator_stake_pool_bump, amount)
    }

    pub fn change_audience_stake(
        ctx: Context<ChangeAudienceStake>,
        membership_bump: u8,
        audience_stake_pool_bump: u8,
        amount: i64,
    ) -> ProgramResult {
        change_audience_stake::handler(ctx, membership_bump, audience_stake_pool_bump, amount)
    }

    pub fn submit_like(ctx: Context<SubmitLike>, like_attr_bump: u8) -> ProgramResult {
        //make like account that stays alive for 10 days
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                &ctx.accounts.member.key(),
                &ctx.accounts.like_attribution.key(),
                calculate_short_term_rent(1, 10),
                1,
                &ctx.program_id,
            ),
            &[
                ctx.accounts.member.to_account_info(),
                ctx.accounts.like_attribution.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&[
                ctx.accounts.membership.key().as_ref(),
                ctx.accounts.post.key.as_ref(),
                &[like_attr_bump],
            ]],
        )?;
        Ok(())
    }

    // pub fn c
    //if i want it to be fully free rolling, i would have to fetch the posts with getProgramAccounts
    //and use the timestamp for each one to get it
    //it's a tradeoff between flexibility and structure
    //is there a way to use a merkle tree to track the likes?

    /*
    --- calculate short term rent
    --- how to find accounts that user is eligible to close?

    */
}

//could pass this in on the client. a bit safer this way tho
fn calculate_short_term_rent(data_len: usize, num_days: u64) -> u64 {
    let __anchor_rent = Rent::get().unwrap();
    let exempt_lamports = Rent::get().unwrap().minimum_balance(data_len);
    exempt_lamports
        .checked_mul(num_days)
        .unwrap()
        .checked_div(730)
        .unwrap()
}

//so here we are just creating a v small account to make sure that posts can't be double liked
#[derive(Accounts)]
#[instruction(like_attr_bump: u8)]
pub struct SubmitLike<'info> {
    pub member: Signer<'info>,
    #[account(
        constraint = membership.authority == member.key()
    )]
    pub membership: Account<'info, Membership>,
    #[account(mut)]
    pub post: AccountInfo<'info>, //change to post once i get there
    #[account(mut,
        seeds = [membership.key().as_ref(), post.key().as_ref()],
        bump = like_attr_bump
    )]
    pub like_attribution: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
