use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use std::convert::TryFrom;

pub mod instructions;
pub mod state;
pub mod utils;
use {instructions::*, state::*, utils::*};

declare_id!("3aGWrcgYM8KPoBU2BFK97UcLFqxMspijPM3o6TgxXog1");

/*
i think my focus for right now should be to build on the anchor metadata and add master, plus get the collections in order
so i should just stick with that for now
and then go back to the cultures
*/

#[program]
pub mod smart_collections {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, smart_authority_bump: u8) -> ProgramResult {
        initialize::handler(ctx, smart_authority_bump)
    }

    pub fn create_collection(
        ctx: Context<CreateCollection>,
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
        create_collection::handler(
            ctx,
            name,
            collection_bump,
            symbol,
            uri,
            mint_authority,
            max_supply,
            creators,
            seller_fee_basis_points,
        )
    }

    pub fn mint_item_into_collection(
        ctx: Context<MintItemIntoCollection>,
        item_name: Option<String>,
        item_symbol: Option<String>,
        item_uri: String,
    ) -> ProgramResult {
        mint_item_into_collection::handler(ctx, item_name, item_symbol, item_uri)
    }
}

/*
create mint
create metadata
create master edition
create mint token into receiver
*/
