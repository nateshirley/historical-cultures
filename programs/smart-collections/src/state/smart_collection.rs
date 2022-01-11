use anchor_lang::prelude::*;

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

#[derive(Default, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Creator {
    pub address: Pubkey,
    // In full percentage points
    pub share: u8,
}
//32 + 1 = 33

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
