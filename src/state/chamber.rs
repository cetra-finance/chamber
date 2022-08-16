//! Module provide protocol agnostic repository for DeFi strategies.

use super::ProtocolType;
use anchor_lang::prelude::*;

#[account]
pub struct Chamber {
    /// Leveraged farm associated with `ProtocolType`.
    pub leveraged_farm: Pubkey,

    /// `Chamber` authority for manage positions.
    pub authority: Pubkey,

    /// Base associated token account.
    pub base_ata: Pubkey,

    /// Quote associated token account.
    pub quote_ata: Pubkey,

    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,

    pub protocol_type: ProtocolType,
    pub bump: u8,
    pub authority_bump: u8,
}

impl Chamber {
    pub const LEN: usize = 8 + (32 + 32 + 32 + 32 + 32 + 32 + 1 + 1 + 1);

    pub fn init(
        &mut self,
        leveraged_farm: &Pubkey,
        authority: &Pubkey,
        base_ata: &Pubkey,
        quote_ata: &Pubkey,
        base_mint: &Pubkey,
        quote_mint: &Pubkey,
        protocol_type: ProtocolType,
        bump: u8,
        authority_bump: u8,
    ) {
        self.leveraged_farm = leveraged_farm.clone();
        self.authority = authority.clone();
        self.base_ata = base_ata.clone();
        self.quote_ata = quote_ata.clone();
        self.base_mint = base_mint.clone();
        self.quote_mint = quote_mint.clone();
        self.protocol_type = protocol_type;
        self.bump = bump;
        self.authority_bump = authority_bump;
    }
}
