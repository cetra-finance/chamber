use crate::InitializeUserPosition;
use anchor_lang::prelude::*;
use anchor_spl::token;

impl<'c, 'info> InitializeUserPosition<'info> {
    pub fn process(&mut self, bump: u8, base_amount: u64, quote_amount: u64) -> Result<()> {
        self.user_position
            .init(self.payer.key, &self.chamber.key(), bump);

        // TODO: Enhance user position management
        // TODO: Add position states
        // TODO: Change exact amounts to interest bearing tokens (percentage)

        // 1. Deposit base amount
        {
            let cpi_accounts = token::Transfer {
                from: self.user_base_ata.to_account_info(),
                to: self.chamber_base_ata.to_account_info(),
                authority: self.payer.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
            token::transfer(cpi_ctx, base_amount)?;

            self.user_position.deposit_base(base_amount)?;
        }

        // 2. Deposit quote amount
        {
            let cpi_accounts = token::Transfer {
                from: self.user_quote_ata.to_account_info(),
                to: self.chamber_quote_ata.to_account_info(),
                authority: self.payer.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
            token::transfer(cpi_ctx, quote_amount)?;

            self.user_position.deposit_quote(quote_amount)?;
        }

        Ok(())
    }
}
