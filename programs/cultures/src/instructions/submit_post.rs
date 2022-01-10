use std::convert::TryFrom;
use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(space: u32)]
pub struct SubmitPost<'info> {
    culture: Account<'info, Culture>,
    member: Signer<'info>,
    #[account(
        mut,
        constraint = membership.culture == culture.key(),
        constraint = membership.member == member.key(),
    )]
    membership: Account<'info, Membership>,
    #[account(
        init,
        space = usize::try_from(space).unwrap(),
        payer = member
    )]
    post: Account<'info, Post>, //also needs to sign tx
    clock: Sysvar<'info, Clock>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SubmitPost>, _space: u32, body: String) -> ProgramResult {
    ctx.accounts.post.culture = ctx.accounts.culture.key();
    ctx.accounts.post.member = ctx.accounts.member.key();
    ctx.accounts.post.body = body;
    ctx.accounts.post.timestamp = u64::try_from(ctx.accounts.clock.unix_timestamp).unwrap();
    Ok(())
}
