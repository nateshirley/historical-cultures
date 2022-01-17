pub mod instruction;
pub mod state;

pub mod anchor_token_metadata {
    use crate::instruction::{
        create_master_edition_v3_ix, create_metadata_v2_ix, verify_collection_ix,
    };
    use crate::state::*;
    use anchor_lang::prelude::*;
    use anchor_lang::solana_program::{self};
    use spl_token_metadata::state::Creator;

    pub fn create_metadata_v2<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreateMetadataV2<'info>>,
        name: String,
        symbol: String,
        uri: String,
        creators: Option<Vec<Creator>>,
        seller_fee_basis_points: u16,
        update_authority_is_signer: bool,
        is_mutable: bool,
        collection: Option<Collection>,
        uses: Option<Uses>,
    ) -> ProgramResult {
        let ix = create_metadata_v2_ix(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.update_authority.key(),
            name,
            symbol,
            uri,
            creators,
            seller_fee_basis_points,
            update_authority_is_signer,
            is_mutable,
            collection,
            uses,
        );
        solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.metadata,
                ctx.accounts.mint,
                ctx.accounts.mint_authority,
                ctx.accounts.payer,
                ctx.accounts.update_authority,
                ctx.accounts.system_program,
                ctx.accounts.rent,
            ],
            ctx.signer_seeds,
        )?;
        Ok(())
    }

    pub fn create_master_edition_v3<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreateMasterEditionV3<'info>>,
        max_supply: Option<u64>,
    ) -> ProgramResult {
        let ix = create_master_edition_v3_ix(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.edition.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.update_authority.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.payer.key(),
            max_supply,
        );
        solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.edition,
                ctx.accounts.mint,
                ctx.accounts.update_authority,
                ctx.accounts.mint_authority,
                ctx.accounts.payer,
                ctx.accounts.metadata,
                ctx.accounts.token_program,
                ctx.accounts.system_program,
                ctx.accounts.rent,
            ],
            ctx.signer_seeds,
        )?;
        Ok(())
    }

    pub fn verify_collection<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, VerifyCollection<'info>>,
    ) -> ProgramResult {
        let ix = verify_collection_ix(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.item_metadata.key(),
            ctx.accounts.collection_authority.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.collection_mint.key(),
            ctx.accounts.collection_metadata.key(),
            ctx.accounts.collection_master_edition.key(),
        );
        solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.item_metadata,
                ctx.accounts.collection_authority,
                ctx.accounts.payer,
                ctx.accounts.collection_mint,
                ctx.accounts.collection_metadata,
                ctx.accounts.collection_master_edition,
            ],
            ctx.signer_seeds,
        )?;
        Ok(())
    }

    #[derive(Accounts)]
    pub struct VerifyCollection<'info> {
        pub item_metadata: AccountInfo<'info>,
        pub collection_authority: AccountInfo<'info>, //update authority on the collection
        pub payer: AccountInfo<'info>,
        pub collection_mint: AccountInfo<'info>,
        pub collection_metadata: AccountInfo<'info>,
        pub collection_master_edition: AccountInfo<'info>,
        #[account(address = spl_token_metadata::id())]
        pub token_metadata_program: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct CreateMasterEditionV3<'info> {
        pub edition: AccountInfo<'info>,
        pub mint: AccountInfo<'info>,
        pub update_authority: AccountInfo<'info>,
        pub mint_authority: AccountInfo<'info>,
        pub payer: AccountInfo<'info>,
        pub metadata: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        #[account(address = spl_token_metadata::id())]
        pub token_metadata_program: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
        pub rent: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct CreateMetadataV2<'info> {
        pub metadata: AccountInfo<'info>, //Metadata key (pda of ['metadata', program id, mint id])
        pub mint: AccountInfo<'info>,     //mint of the token we are creating metadata for
        pub mint_authority: AccountInfo<'info>,
        pub payer: AccountInfo<'info>,
        pub update_authority: AccountInfo<'info>, //this is the account that will have future ability to update the newly created metadata
        #[account(address = spl_token_metadata::id())]
        pub token_metadata_program: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
        pub rent: AccountInfo<'info>,
    }

    /*


    pub fn create_metadata<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreateMetadata<'info>>,
        name: String,
        symbol: String,
        uri: String,
        creators: Option<Vec<Creator>>,
        seller_fee_basis_points: u16,
        update_authority_is_signer: bool,
        is_mutable: bool,
    ) -> ProgramResult {
        let ix = create_metadata_ix(
            *ctx.accounts.token_metadata_program.key,
            *ctx.accounts.metadata.key,
            ctx.accounts.mint.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.update_authority.key(),
            name,
            symbol,
            uri,
            creators,
            seller_fee_basis_points,
            update_authority_is_signer,
            is_mutable,
        );
        solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.metadata.clone(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.update_authority.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
            ctx.signer_seeds,
        )?;
        Ok(())
    }

        pub fn update_metadata<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateMetadataAccount<'info>>,
        new_update_authority: Option<Pubkey>,
        data: Option<Data>,
        primary_sale_happened: Option<bool>,
    ) -> ProgramResult {
        let ix = update_metadata_accounts(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.update_authority.key(),
            new_update_authority,
            data,
            primary_sale_happened,
        );
        solana_program::program::invoke_signed(
            &ix,
            &[ctx.accounts.metadata, ctx.accounts.update_authority],
            ctx.signer_seeds,
        )?;
        Ok(())
    }
    #[derive(Accounts)]
    pub struct UpdateMetadataAccount<'info> {
        #[account(mut)]
        pub metadata: AccountInfo<'info>,
        pub update_authority: AccountInfo<'info>,
        #[account(address = spl_token_metadata::id())]
        pub token_metadata_program: AccountInfo<'info>,
    }

    /// Create Metadata object.
    ///   0. `[writable]`  Metadata key (pda of ['metadata', program id, mint id])
    ///   1. `[]` Mint of token asset
    ///   2. `[signer]` Mint authority
    ///   3. `[signer]` payer
    ///   4. `[]` update authority info
    ///   5. `[]` System program
    ///   6. `[]` Rent info
    //CreateMetadataAccountV2(CreateMetadataAccountArgsV2)


    #[allow(clippy::too_many_arguments)]
    pub fn create_metadata_ix(
        program_id: Pubkey,
        metadata_account: Pubkey,
        mint: Pubkey,
        mint_authority: Pubkey,
        payer: Pubkey,
        update_authority: Pubkey,
        name: String,
        symbol: String,
        uri: String,
        creators: Option<Vec<Creator>>,
        seller_fee_basis_points: u16,
        update_authority_is_signer: bool,
        is_mutable: bool,
    ) -> Instruction {
        Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(metadata_account, false),
                AccountMeta::new_readonly(mint, false),
                AccountMeta::new_readonly(mint_authority, true),
                AccountMeta::new(payer, true),
                AccountMeta::new_readonly(update_authority, update_authority_is_signer),
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
                AccountMeta::new_readonly(sysvar::rent::id(), false),
            ],
            data: MetadataInstruction::CreateMetadataAccount(CreateMetadataAccountArgs {
                data: Data {
                    name,
                    symbol,
                    uri,
                    seller_fee_basis_points,
                    creators,
                },
                is_mutable,
            })
            .try_to_vec()
            .unwrap(),
        }
    }
    */
}
