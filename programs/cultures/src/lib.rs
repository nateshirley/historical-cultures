use anchor_lang::prelude::*;

declare_id!("5prqgWu5iSVS8DvYruAFFCNQjAbg1RvojMCRL6dPSPye");

mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

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

    pub fn submit_like(ctx: Context<SubmitLike>) -> ProgramResult {
        Ok(())
    }

    // pub fn c
    //if i want it to be fully free rolling, i would have to fetch the posts with getProgramAccounts
    //and use the timestamp for each one to get it
    //it's a tradeoff between flexibility and structure
    //is there a way to use a merkle tree to track the likes?
}

#[derive(Accounts)]
pub struct SubmitLike<'info> {
    pub member: Signer<'info>,
    pub post: AccountInfo<'info>,
    #[account(
        init,
        seeds = [post.key().as_ref()],
        bump,
        payer = member,
        space = 1
    )]
    pub like_attribution: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
