use anchor_lang::prelude::*;

pub const CHAMBER_PREFIX: &str = "chamber";
pub const CHAMBER_AUTHORITY_PREFIX: &str = "chamber_authority";
pub const USER_POSITION_PREFIX: &str = "user_position";

pub fn derive_chamber_address(leveraged_farm: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[CHAMBER_PREFIX.as_bytes(), leveraged_farm.as_ref()],
        &crate::id(),
    )
}

pub fn derive_chamber_authority(chamber: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[CHAMBER_AUTHORITY_PREFIX.as_bytes(), chamber.as_ref()],
        &crate::id(),
    )
}

pub fn derive_user_position(wallet: &Pubkey, chamber: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            USER_POSITION_PREFIX.as_bytes(),
            wallet.as_ref(),
            chamber.as_ref(),
        ],
        &crate::id(),
    )
}
