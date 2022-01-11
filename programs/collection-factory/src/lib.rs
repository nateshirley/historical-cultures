use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use std::convert::TryFrom;
declare_id!("CPbfVzeuKJczvj149dFnrUMqHyZWnSimzHouQ4X43x5H");

const COLLECTION_SEED: &[u8] = b"collection";
const FACTORY_AUTHORITY_SEED: &[u8] = b"f_auth";

#[program]
pub mod collection_factory {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, factory_auth_bump: u8) -> ProgramResult {
        ctx.accounts.factory_authority.bump = factory_auth_bump;
        Ok(())
    }

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        name: String,
        collection_bump: u8,
        _space: u16,
        symbol: String,
        mint_authority: Pubkey,
        max_supply: Option<u32>,
        creators: Option<Vec<Creator>>,
        seller_fee_basis_points: u16,
    ) -> ProgramResult {
        let collection = &mut ctx.accounts.collection;
        collection.name = name.to_seed_format();
        collection.symbol = symbol;
        collection.mint_authority = Some(mint_authority);
        collection.max_supply = max_supply;
        collection.creators = creators;
        collection.seller_fee_basis_points = seller_fee_basis_points;
        collection.bump = collection_bump;
        Ok(())
    }

    pub fn mint_into_collection(ctx: Context<MintIntoCollection>) -> ProgramResult {
        let seeds = &[
            &FACTORY_AUTHORITY_SEED[..],
            &[ctx.accounts.factory_authority.bump],
        ];
        token::mint_to(
            ctx.accounts
                .into_mint_item_to_receiver_context()
                .with_signer(&[seeds]),
            1,
        )?;

        ctx.accounts.collection.supply = ctx.accounts.collection.supply.checked_add(1).unwrap();
        //mint one token to the receiver
        //create metadata for the mint
        //create master edition
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(factory_auth_bump: u8)]
pub struct Initialize<'info> {
    initializer: Signer<'info>,
    #[account(
        init,
        seeds = [FACTORY_AUTHORITY_SEED],
        bump = factory_auth_bump,
        payer = initializer
    )]
    factory_authority: Account<'info, Authority>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String, collection_bump: u8, space: u16)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    creator: Signer<'info>,
    #[account(
        init,
        seeds = [COLLECTION_SEED, name.clone().to_seed_format().as_bytes()],
        bump = collection_bump,
        payer = creator,
        space = usize::try_from(space).unwrap()
    )]
    collection: Account<'info, Collection>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintIntoCollection<'info> {
    #[account(
        mut,
        constraint = collection.mint_authority.unwrap() == collection_mint_authority.key(),
        constraint = collection.has_remaining_supply()
    )]
    collection: Account<'info, Collection>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = factory_authority,
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
        seeds = [FACTORY_AUTHORITY_SEED],
        bump = factory_authority.bump
    )]
    factory_authority: Account<'info, Authority>,
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, token::Token>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
    system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Collection {
    pub name: String,
    pub symbol: String,
    pub mint_authority: Option<Pubkey>,
    pub supply: u32,
    pub max_supply: Option<u32>,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct Authority {
    bump: u8,
}

impl<'info> MintIntoCollection<'info> {
    fn into_mint_item_to_receiver_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::MintTo {
            mint: self.item_mint.to_account_info(),
            to: self.receiver_token_account.to_account_info(),
            authority: self.factory_authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub trait MaxSupplyConstraints {
    fn has_remaining_supply(&self) -> bool;
}
impl MaxSupplyConstraints for Collection {
    fn has_remaining_supply(&self) -> bool {
        if let Some(max_supply) = self.max_supply {
            self.supply < max_supply
        } else {
            true
        }
    }
}

//8
//4 + str len
//4 + str len
//33
//4
//5
//2
//1 + (34 * creator_len)
//1
//= 62 + (name bytes) + (symbol bytes) + (33 * creator_len)

#[derive(Default, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Creator {
    pub address: Pubkey,
    // In full percentage points
    pub share: u8,
}
//32 + 1 = 33

pub trait SeedFormat {
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
