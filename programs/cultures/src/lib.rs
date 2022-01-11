use anchor_lang::prelude::*;
declare_id!("54msrd3yQCZjPdPB6Fh95yp7xYq8NiWgbWm1w2WuGdSA");

mod instructions;
pub mod state;
pub mod utils;

use anchor_spl::token;
use instructions::*;
use state::*;
use utils::*;
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

    pub fn create_post(ctx: Context<CreatePost>, space: u32, body: String) -> ProgramResult {
        create_post::handler(ctx, space, body)
    }

    pub fn like_post(ctx: Context<LikePost>, like_attr_bump: u8) -> ProgramResult {
        like_post::handler(ctx, like_attr_bump)
    }

    pub fn mint_post(
        ctx: Context<MintPost>,
        _creator_stake_pool_bump: u8,
        _audience_stake_pool_bump: u8,
    ) -> ProgramResult {
        mint_post::handler(ctx)
    }
}

/*
some shit i need to do
- how to delete old posts
- minting
- collection factory
*/

/*
why do u even need the posts on chain?

main reason is u need to keep track of whose votes are staked into what posts
if u don't do it on-chain, u can't really verify whose tokens are on what posts
still relatively low risk actually, bc the posts are just getting minted for free, no value locked really
*/
