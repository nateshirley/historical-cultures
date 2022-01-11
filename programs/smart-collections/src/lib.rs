use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use std::convert::TryFrom;

pub mod instructions;
pub mod state;
pub mod utils;
use {instructions::*, state::*, utils::*};

declare_id!("3aGWrcgYM8KPoBU2BFK97UcLFqxMspijPM3o6TgxXog1");

#[program]
pub mod smart_collections {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, smart_authority_bump: u8) -> ProgramResult {
        initialize::handler(ctx, smart_authority_bump)
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
        create::handler(
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

    pub fn mint_into(ctx: Context<MintInto>) -> ProgramResult {
        mint_into::handler(ctx)
    }
}

/*
create mint
create metadata
create master edition
create mint token into receiver
*/
