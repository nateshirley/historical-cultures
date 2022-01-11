use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use anchor_spl_token_metadata::anchor_token_metadata;
use spl_token_metadata;
use std::convert::TryFrom;

declare_id!("3aGWrcgYM8KPoBU2BFK97UcLFqxMspijPM3o6TgxXog1");

const COLLECTION_SEED: &[u8] = b"collection";
const SMART_AUTHORITY_SEED: &[u8] = b"s_auth";

#[program]
pub mod smart_collections {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, smart_authority_bump: u8) -> ProgramResult {
        ctx.accounts.smart_authority.bump = smart_authority_bump;
        Ok(())
    }

    pub fn create(
        ctx: Context<Create>,
        name: String,
        collection_bump: u8,
        _space: u16,
        symbol: String,
        uri: String,
        mint_authority: Pubkey,
        max_supply: Option<u32>,
        creators: Option<Vec<Creator>>,
        seller_fee_basis_points: u16,
    ) -> ProgramResult {
        //config smart collection
        let smart_collection = &mut ctx.accounts.smart_collection;
        smart_collection.name = name.clone().to_seed_format();
        smart_collection.symbol = symbol.clone();
        smart_collection.mint_authority = Some(mint_authority);
        smart_collection.max_supply = max_supply;
        smart_collection.creators = creators.clone();
        smart_collection.seller_fee_basis_points = seller_fee_basis_points;
        smart_collection.bump = collection_bump;

        //mint collection token to smart authority
        let seeds = &[
            &SMART_AUTHORITY_SEED[..],
            &[ctx.accounts.smart_authority.bump],
        ];
        token::mint_to(
            ctx.accounts
                .into_mint_collection_token_to_authority_context()
                .with_signer(&[seeds]),
            1,
        )?;

        //create metadata for the collection
        // anchor_token_metadata::create_metadata(
        //     ctx.accounts
        //         .into_create_collection_metadata_context()
        //         .with_signer(&[&seeds[..]]),
        //     name.to_name_format(),
        //     symbol,
        //     uri,
        //     to_metaplex_metadata_creators(creators),
        //     0,
        //     true,
        //     true,
        // )?;

        //max supply zero for master edition

        //create master edition for the collection
        //update metadata w/ primary sale happened
        Ok(())
    }

    pub fn mint_into(ctx: Context<MintInto>) -> ProgramResult {
        let seeds = &[
            &SMART_AUTHORITY_SEED[..],
            &[ctx.accounts.smart_authority.bump],
        ];
        token::mint_to(
            ctx.accounts
                .into_mint_item_to_receiver_context()
                .with_signer(&[seeds]),
            1,
        )?;

        ctx.accounts.smart_collection.supply =
            ctx.accounts.smart_collection.supply.checked_add(1).unwrap();
        //mint one token to the receiver
        //create metadata for the mint
        //create master edition
        Ok(())
    }
}

fn to_metaplex_metadata_creators(
    creators: Option<Vec<Creator>>,
) -> Option<Vec<spl_token_metadata::state::Creator>> {
    if let Some(creators) = creators {
        let mut metaplex_creators: Vec<spl_token_metadata::state::Creator> = Vec::new();
        for creator in creators.iter() {
            let metaplex_creator = spl_token_metadata::state::Creator {
                address: creator.address,
                verified: false,
                share: creator.share,
            };
            metaplex_creators.push(metaplex_creator);
        }
        Some(metaplex_creators)
    } else {
        None
    }
}

#[derive(Accounts)]
#[instruction(smart_authority_bump: u8)]
pub struct Initialize<'info> {
    initializer: Signer<'info>,
    #[account(
        init,
        seeds = [SMART_AUTHORITY_SEED],
        bump = smart_authority_bump,
        payer = initializer
    )]
    smart_authority: Account<'info, Authority>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String, collection_bump: u8, space: u16)]
pub struct Create<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(
        init,
        seeds = [COLLECTION_SEED, name.clone().to_seed_format().as_bytes()],
        bump = collection_bump,
        payer = payer,
        space = usize::try_from(space).unwrap()
    )]
    smart_collection: Account<'info, SmartCollection>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = smart_authority,
    )]
    collection_mint: Account<'info, token::Mint>, //must also be signer,
    collection_metadata: UncheckedAccount<'info>, //validated via cpi
    collection_master_edition: UncheckedAccount<'info>, //validated via cpi
    #[account(
        init,
        payer = payer,
        associated_token::authority = smart_authority,
        associated_token::mint = collection_mint,
    )]
    collection_token_account: Account<'info, token::TokenAccount>,
    #[account(
        seeds = [SMART_AUTHORITY_SEED],
        bump = smart_authority.bump
    )]
    smart_authority: Account<'info, Authority>,
    rent: Sysvar<'info, Rent>,
    token_metadata_program: AccountInfo<'info>, //Program<'info, anchor_token_metadata::TokenMetadata>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
    token_program: Program<'info, token::Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintInto<'info> {
    #[account(
        mut,
        constraint = smart_collection.mint_authority.unwrap() == collection_mint_authority.key(),
        constraint = smart_collection.has_remaining_supply()
    )]
    smart_collection: Account<'info, SmartCollection>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = smart_authority,
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
        seeds = [SMART_AUTHORITY_SEED],
        bump = smart_authority.bump
    )]
    smart_authority: Account<'info, Authority>,
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, token::Token>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
    system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct SmartCollection {
    pub mint: Pubkey,
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

impl<'info> Create<'info> {
    fn into_mint_collection_token_to_authority_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::MintTo {
            mint: self.collection_mint.to_account_info(),
            to: self.collection_token_account.to_account_info(),
            authority: self.smart_authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
    fn into_create_collection_metadata_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, anchor_token_metadata::CreateMetadata<'info>> {
        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = anchor_token_metadata::CreateMetadata {
            metadata: self.collection_metadata.to_account_info(),
            mint: self.collection_mint.to_account_info(),
            mint_authority: self.smart_authority.to_account_info(),
            payer: self.payer.to_account_info(),
            update_authority: self.smart_authority.to_account_info(),
            token_metadata_program: self.token_metadata_program.to_account_info(),
            system_program: self.system_program.clone(),
            rent: self.rent.clone(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> MintInto<'info> {
    fn into_mint_item_to_receiver_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::MintTo {
            mint: self.item_mint.to_account_info(),
            to: self.receiver_token_account.to_account_info(),
            authority: self.smart_authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub trait SupplyConstraints {
    fn has_remaining_supply(&self) -> bool;
}
impl SupplyConstraints for SmartCollection {
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
pub trait NameFormat {
    fn to_name_format(self) -> String;
}
impl NameFormat for String {
    fn to_name_format(mut self) -> String {
        self.retain(|c| !c.is_whitespace());
        self
    }
}

/*
create mint
create metadata
create master edition
create mint token into receiver
*/
