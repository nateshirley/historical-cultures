use anchor_lang::solana_program;
use anchor_spl::{associated_token, token};
use anchor_spl_token_metadata::anchor_token_metadata;
use std::convert::TryFrom;
use {crate::state::*, crate::utils::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(name: String, collection_bump: u8, space: u16)]
pub struct CreateCollection<'info> {
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
    #[account(mut)]
    collection_metadata: UncheckedAccount<'info>, //validated via cpi
    #[account(mut)]
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

pub fn handler(
    ctx: Context<CreateCollection>,
    name: String,
    collection_bump: u8,
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
    smart_collection.mint = ctx.accounts.collection_mint.key();
    smart_collection.mint_authority = Some(mint_authority);
    smart_collection.max_supply = max_supply;
    smart_collection.creators = creators.clone();
    smart_collection.seller_fee_basis_points = seller_fee_basis_points;
    smart_collection.bump = collection_bump;

    let seeds = &[
        &SMART_AUTHORITY_SEED[..],
        &[ctx.accounts.smart_authority.bump],
    ];

    //mint collection token to smart authority
    token::mint_to(
        ctx.accounts
            .into_mint_collection_token_to_authority_context()
            .with_signer(&[seeds]),
        1,
    )?;

    //create metadata for the collection
    anchor_token_metadata::create_metadata_v2(
        ctx.accounts
            .into_create_collection_metadata_context()
            .with_signer(&[seeds]),
        name.to_name_format(),
        symbol,
        uri,
        to_metaplex_creators(creators), //still have to fix creators issue but moving to master rn
        0,
        true,
        true,
        None,
        None,
    )?;

    //create master edition for the collection
    anchor_token_metadata::create_master_edition_v3(
        ctx.accounts
            .into_create_collection_master_edition_context()
            .with_signer(&[seeds]),
        Some(0),
    )?;

    //update metadata w/ primary sale happened
    //todo
    Ok(())
}
impl<'info> CreateCollection<'info> {
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
    ) -> CpiContext<'_, '_, '_, 'info, anchor_token_metadata::CreateMetadataV2<'info>> {
        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = anchor_token_metadata::CreateMetadataV2 {
            metadata: self.collection_metadata.to_account_info(),
            mint: self.collection_mint.to_account_info(),
            mint_authority: self.smart_authority.to_account_info(),
            payer: self.payer.to_account_info(),
            update_authority: self.smart_authority.to_account_info(),
            token_metadata_program: self.token_metadata_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
    fn into_create_collection_master_edition_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, anchor_token_metadata::CreateMasterEditionV3<'info>> {
        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = anchor_token_metadata::CreateMasterEditionV3 {
            edition: self.collection_master_edition.to_account_info(),
            mint: self.collection_mint.to_account_info(),
            update_authority: self.smart_authority.to_account_info(),
            mint_authority: self.smart_authority.to_account_info(),
            payer: self.payer.to_account_info(),
            metadata: self.collection_metadata.to_account_info(),
            token_program: self.token_program.to_account_info(),
            token_metadata_program: self.token_metadata_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// pub edition: AccountInfo<'info>,
// pub mint: AccountInfo<'info>,
// pub update_authority: AccountInfo<'info>,
// pub mint_authority: AccountInfo<'info>,
// pub payer: AccountInfo<'info>,
// pub metadata: AccountInfo<'info>,
// pub token_program: AccountInfo<'info>,
// #[account(address = spl_token_metadata::id())]
// pub token_metadata_program: AccountInfo<'info>,
// pub system_program: AccountInfo<'info>,
// pub rent: AccountInfo<'info>,

// let ix = anchor_token_metadata::create_metadata_v2_ix(
//     *ctx.accounts.token_metadata_program.key,
//     *ctx.accounts.collection_metadata.key,
//     ctx.accounts.collection_mint.key(),
//     ctx.accounts.smart_authority.key(),
//     ctx.accounts.payer.key(),
//     ctx.accounts.smart_authority.key(),
//     name.to_name_format(),
//     symbol,
//     uri,
//     None,
//     0,
//     true,
//     false,
//     None,
//     None,
// );
// solana_program::program::invoke_signed(
//     &ix,
//     &[
//         ctx.accounts.collection_metadata.to_account_info(),
//         ctx.accounts.collection_mint.to_account_info(),
//         ctx.accounts.smart_authority.to_account_info(),
//         ctx.accounts.payer.to_account_info(),
//         ctx.accounts.smart_authority.to_account_info(),
//         ctx.accounts.system_program.to_account_info(),
//         ctx.accounts.rent.to_account_info(),
//     ],
//     &[seeds],
// )?;
