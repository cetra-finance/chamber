//! Module define representation of user position.

use crate::ChamberError;
use anchor_lang::prelude::*;
use std::result::Result;

#[account]
pub struct UserPosition {
    pub owner: Pubkey,
    pub chamber: Pubkey,
    pub base_amount: u64,
    pub quote_amount: u64,
    pub bump: u8,
}

impl UserPosition {
    pub const LEN: usize = 8 + (32 + 32 + 8 + 8 + 1);

    pub fn init(&mut self, owner: &Pubkey, chamber: &Pubkey, bump: u8) {
        self.owner = owner.clone();
        self.chamber = chamber.clone();
        self.base_amount = 0;
        self.quote_amount = 0;
        self.bump = bump;
    }

    pub fn deposit_base(&mut self, amount: u64) -> Result<(), ChamberError> {
        Ok(self.base_amount = self
            .base_amount
            .checked_add(amount)
            .ok_or(ChamberError::MathOverflow)?)
    }

    pub fn withdraw_base(&mut self, amount: u64) -> Result<(), ChamberError> {
        Ok(self.base_amount = self
            .base_amount
            .checked_sub(amount)
            .ok_or(ChamberError::MathOverflow)?)
    }

    pub fn deposit_quote(&mut self, amount: u64) -> Result<(), ChamberError> {
        Ok(self.quote_amount = self
            .quote_amount
            .checked_add(amount)
            .ok_or(ChamberError::MathOverflow)?)
    }

    pub fn withdraw_quote(&mut self, amount: u64) -> Result<(), ChamberError> {
        Ok(self.quote_amount = self
            .quote_amount
            .checked_sub(amount)
            .ok_or(ChamberError::MathOverflow)?)
    }
}
