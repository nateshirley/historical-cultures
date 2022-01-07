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
}
