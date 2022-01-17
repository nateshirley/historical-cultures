use anchor_spl::{associated_token, token};
use anchor_spl_token_metadata::anchor_token_metadata;
use {crate::state::*, crate::utils::*, anchor_lang::prelude::*};
//works
#[derive(Accounts)]
pub struct MintItemIntoCollection<'info> {
    #[account(
        mut,
        constraint = smart_collection.mint == collection_mint.key(),
        constraint = smart_collection.has_remaining_supply()
    )]
    smart_collection: Account<'info, SmartCollection>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = smart_authority,
    )]
    item_mint: Box<Account<'info, token::Mint>>, //must also be signer,
    #[account(mut)]
    item_metadata: UncheckedAccount<'info>, //checked via cpi,
    #[account(mut)]
    item_master_edition: UncheckedAccount<'info>, //checked via cpi
    receiver: AccountInfo<'info>,
    #[account(
        init,
        payer = payer,
        associated_token::authority = receiver,
        associated_token::mint = item_mint,
    )]
    receiver_token_account: Box<Account<'info, token::TokenAccount>>,
    collection_mint: AccountInfo<'info>,
    collection_metadata: UncheckedAccount<'info>,
    collection_master_edition: UncheckedAccount<'info>,
    #[account(
        mut,
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
    ctx: Context<MintItemIntoCollection>,
    item_name: Option<String>,
    item_symbol: Option<String>,
    item_uri: String,
) -> ProgramResult {
    let seeds = &[
        &SMART_AUTHORITY_SEED[..],
        &[ctx.accounts.smart_authority.bump],
    ];
    //mint one token to the receiver
    token::mint_to(
        ctx.accounts
            .into_mint_item_to_receiver_context()
            .with_signer(&[seeds]),
        1,
    )?;
    //derive metadata from collection if not given
    let metadata_name = if let Some(name) = item_name {
        name.to_name_format()
    } else {
        ctx.accounts.smart_collection.name.clone()
    };
    let metadata_symbol = if let Some(symbol) = item_symbol {
        symbol
    } else {
        ctx.accounts.smart_collection.symbol.clone()
    };
    //create metadata for the item
    let collection = anchor_spl_token_metadata::state::Collection {
        verified: false,
        key: ctx.accounts.smart_collection.mint,
    };
    anchor_token_metadata::create_metadata_v2(
        ctx.accounts
            .into_create_item_metadata_context()
            .with_signer(&[&seeds[..]]),
        metadata_name,
        metadata_symbol,
        item_uri,
        to_metaplex_creators(ctx.accounts.smart_collection.creators.clone()),
        ctx.accounts.smart_collection.seller_fee_basis_points,
        true,
        true,
        Some(collection),
        None,
    )?;

    //create master edition
    anchor_token_metadata::create_master_edition_v3(
        ctx.accounts
            .into_create_item_master_edition_context()
            .with_signer(&[seeds]),
        Some(0),
    )?;

    //verify item's place in the collection
    anchor_token_metadata::verify_collection(
        ctx.accounts
            .into_verify_collection_context()
            .with_signer(&[seeds]),
    )?;

    //update collection supply
    ctx.accounts.smart_collection.supply =
        ctx.accounts.smart_collection.supply.checked_add(1).unwrap();
    Ok(())
}

impl<'info> MintItemIntoCollection<'info> {
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
    fn into_create_item_metadata_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, anchor_token_metadata::CreateMetadataV2<'info>> {
        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = anchor_token_metadata::CreateMetadataV2 {
            metadata: self.item_metadata.to_account_info(),
            mint: self.item_mint.to_account_info(),
            mint_authority: self.smart_authority.to_account_info(),
            payer: self.payer.to_account_info(),
            update_authority: self.smart_authority.to_account_info(),
            token_metadata_program: self.token_metadata_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
    fn into_create_item_master_edition_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, anchor_token_metadata::CreateMasterEditionV3<'info>> {
        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = anchor_token_metadata::CreateMasterEditionV3 {
            edition: self.item_master_edition.to_account_info(),
            mint: self.item_mint.to_account_info(),
            update_authority: self.smart_authority.to_account_info(),
            mint_authority: self.smart_authority.to_account_info(),
            payer: self.payer.to_account_info(),
            metadata: self.item_metadata.to_account_info(),
            token_program: self.token_program.to_account_info(),
            token_metadata_program: self.token_metadata_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
    fn into_verify_collection_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, anchor_token_metadata::VerifyCollection<'info>> {
        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = anchor_token_metadata::VerifyCollection {
            item_metadata: self.item_metadata.to_account_info(),
            collection_authority: self.smart_authority.to_account_info(),
            payer: self.payer.to_account_info(),
            collection_mint: self.collection_mint.to_account_info(),
            collection_metadata: self.collection_metadata.to_account_info(),
            collection_master_edition: self.collection_master_edition.to_account_info(),
            token_metadata_program: self.token_metadata_program.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// let ix = anchor_spl_token_metadata::instruction::verify_collection_ix(
//     ctx.accounts.token_metadata_program.key(),
//     ctx.accounts.item_metadata.key(),
//     ctx.accounts.smart_authority.key(),
//     ctx.accounts.payer.key(),
//     ctx.accounts.collection_mint.key(),
//     ctx.accounts.collection_metadata.key(),
//     ctx.accounts.collection_master_edition.key(),
// );
// anchor_lang::solana_program::program::invoke_signed(
//     &ix,
//     &[
//         ctx.accounts.item_metadata.to_account_info(),
//         ctx.accounts.smart_authority.to_account_info(),
//         ctx.accounts.payer.to_account_info(),
//         ctx.accounts.collection_mint.to_account_info(),
//         ctx.accounts.collection_metadata.to_account_info(),
//         ctx.accounts.collection_master_edition.to_account_info(),
//     ],
//     &[seeds],
// )?;
