use anchor_lang::prelude::*;
use anchor_spl::token;
use std::convert::TryFrom;
declare_id!("5prqgWu5iSVS8DvYruAFFCNQjAbg1RvojMCRL6dPSPye");

const CULTURE_SEED: &[u8] = b"culture";
const STAKE_AUTHORITY_SEED: &[u8] = b"stake";
const MEMBERSHIP_SEED: &[u8] = b"membership";
const C_REDEMPTION_SEED: &[u8] = b"c_redemption";
const C_STAKE_SEED: &[u8] = b"c_stake";
const A_REDEMPTION_SEED: &[u8] = b"a_redemption";
const A_STAKE_SEED: &[u8] = b"a_stake";

#[program]
pub mod cultures {
    use super::*;

    pub fn initialize_program(
        ctx: Context<InitializeProgram>,
        stake_authority_bump: u8,
    ) -> ProgramResult {
        ctx.accounts.stake_authority.bump = stake_authority_bump;
        Ok(())
    }

    pub fn create_culture(
        ctx: Context<CreateCulture>,
        _culture_bump: u8,
        name: String,
    ) -> ProgramResult {
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

    pub fn create_membership(ctx: Context<CreateMembership>, membership_bump: u8) -> ProgramResult {
        //config membership
        ctx.accounts.membership.culture = ctx.accounts.culture.key();
        ctx.accounts.membership.authority = ctx.accounts.new_member.key();
        ctx.accounts.membership.bump = membership_bump;
        Ok(())
    }

    pub fn change_creator_stake(
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

    pub fn change_audience_stake(
        ctx: Context<ChangeAudienceStake>,
        _membership_bump: u8,
        _audience_stake_pool_bump: u8,
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
                        from: ctx.accounts.audience_token_account.to_account_info(),
                        to: ctx.accounts.audience_stake_pool.to_account_info(),
                        authority: ctx.accounts.member.to_account_info(),
                    },
                ),
                unsigned_amount,
            )?;
            new_stake = ctx
                .accounts
                .membership
                .audience_stake
                .checked_add(unsigned_amount)
                .unwrap();
        } else {
            let unsigned_amount = u64::try_from(amount.checked_abs().unwrap()).unwrap();
            if unsigned_amount <= ctx.accounts.membership.audience_stake {
                anchor_spl::token::transfer(
                    CpiContext::new(
                        ctx.accounts.token_program.to_account_info(),
                        anchor_spl::token::Transfer {
                            from: ctx.accounts.audience_stake_pool.to_account_info(),
                            to: ctx.accounts.audience_token_account.to_account_info(),
                            authority: ctx.accounts.stake_authority.to_account_info(),
                        },
                    )
                    .with_signer(&[&[STAKE_AUTHORITY_SEED, &[ctx.accounts.stake_authority.bump]]]),
                    unsigned_amount,
                )?;
                new_stake = ctx
                    .accounts
                    .membership
                    .audience_stake
                    .checked_sub(unsigned_amount)
                    .unwrap();
            } else {
                return Err(ErrorCode::InsufficientStakeWithdraw.into());
            }
        }
        //reflect changes in membership account
        ctx.accounts.membership.audience_stake = new_stake;
        Ok(())
    }
}

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

#[derive(Accounts)]
#[instruction(membership_bump: u8, audience_stake_pool_bump: u8)]
pub struct ChangeAudienceStake<'info> {
    member: Signer<'info>,
    // #[account(
    //     constraint = culture.creator_mint != culture.audience_mint
    // )]
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
        constraint = audience_token_account.owner == member.key(),
        constraint = audience_token_account.mint == culture.creator_mint,
    )]
    audience_token_account: Account<'info, token::TokenAccount>,
    #[account(
        mut,
        constraint = audience_stake_pool.key() == find_audience_stake_pool(culture.key(), audience_stake_pool_bump)
    )]
    audience_stake_pool: Account<'info, token::TokenAccount>,
    #[account(
        seeds = [STAKE_AUTHORITY_SEED],
        bump = stake_authority.bump,
    )]
    stake_authority: Account<'info, Authority>,
    token_program: Program<'info, token::Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(stake_authority_bump: u8)]
pub struct InitializeProgram<'info> {
    initializer: Signer<'info>,
    #[account(
        init,
        seeds = [STAKE_AUTHORITY_SEED],
        bump = stake_authority_bump,
        payer = initializer
    )]
    stake_authority: Account<'info, Authority>,
    system_program: Program<'info, System>,
}

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

//i need to decide if im going to use mint or token
#[derive(Accounts)]
#[instruction(membership_bump: u8)]
pub struct CreateMembership<'info> {
    new_member: Signer<'info>,
    culture: Account<'info, Culture>,
    #[account(
        init_if_needed,
        seeds = [MEMBERSHIP_SEED, culture.key().as_ref(), new_member.key().as_ref()],
        bump = membership_bump,
        payer = new_member
    )]
    membership: Account<'info, Membership>,
    system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Culture {
    name: String,
    collection: Pubkey,
    treasury: Pubkey,
    creator_mint: Pubkey,
    creator_count: u32,
    audience_mint: Pubkey,
    audience_count: u32,
    bump: u8,
}
//8 + str + (32 * 4) + 4 + 1
// = 141 + str
//str = 20 (16chars + 4 setup)

#[account]
#[derive(Default)]
pub struct Membership {
    culture: Pubkey,
    authority: Pubkey,
    creator_stake: u64,
    audience_stake: u64,
    bump: u8,
}

//create one time stake auth that controls all stake mints
#[account]
#[derive(Default)]
pub struct Authority {
    bump: u8,
}

trait SeedFormat {
    fn to_seed_format(self) -> String;
}
//need to add some error handling to the front end
impl SeedFormat for String {
    //checks for length and special chars
    fn to_seed_format(mut self) -> String {
        self.make_ascii_lowercase();
        self.retain(|c| !c.is_whitespace());
        self
    }
}

pub fn find_creator_stake_pool(culture: Pubkey, bump: u8) -> Pubkey {
    Pubkey::create_program_address(&[C_STAKE_SEED, culture.key().as_ref(), &[bump]], &id()).unwrap()
}
pub fn find_audience_stake_pool(culture: Pubkey, bump: u8) -> Pubkey {
    Pubkey::create_program_address(&[A_STAKE_SEED, culture.key().as_ref(), &[bump]], &id()).unwrap()
}

#[error]
pub enum ErrorCode {
    #[msg("you are trying to unstake more than you have staked")]
    InsufficientStakeWithdraw,
}
