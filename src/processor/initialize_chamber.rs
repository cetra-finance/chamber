use crate::{state, utils, ChamberError, InitializeChamber};
use anchor_lang::prelude::*;
use anchor_spl::associated_token;

impl<'c, 'info> InitializeChamber<'info> {
    pub fn process(
        &mut self,
        leveraged_farm: Pubkey,
        bump: u8,
        authority_bump: u8,
        protocol_type: state::ProtocolType,
    ) -> Result<()> {
        // TODO: Extend protocols support
        if protocol_type != state::ProtocolType::Tulip {
            return Err(ChamberError::UnsupportedProtocol.into());
        }

        // 1. Initialize chamber
        self.chamber.init(
            &leveraged_farm,
            &self.authority.key(),
            &self.base_ata.key(),
            &self.quote_ata.key(),
            &self.base_mint.key(),
            &self.quote_mint.key(),
            protocol_type,
            bump,
            authority_bump,
        );

        let chamber_pubkey = self.chamber.key();

        let seeds: &[&[&[u8]]] = &[&[
            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
            chamber_pubkey.as_ref(),
            &[self.chamber.authority_bump],
        ]];

        // 2. Create base ata
        {
            let cpi_accounts = associated_token::Create {
                payer: self.payer.to_account_info(),
                associated_token: self.base_ata.to_account_info(),
                authority: self.authority.to_account_info(),
                mint: self.base_mint.to_account_info(),
                rent: self.rent_sysvar.to_account_info(),
                token_program: self.token_program.to_account_info(),
                system_program: self.system_program.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                cpi_accounts,
                seeds,
            );
            associated_token::create(cpi_ctx)?;
        }

        // 3. Create quote ata
        {
            let cpi_accounts = associated_token::Create {
                payer: self.payer.to_account_info(),
                associated_token: self.quote_ata.to_account_info(),
                authority: self.authority.to_account_info(),
                mint: self.quote_mint.to_account_info(),
                rent: self.rent_sysvar.to_account_info(),
                token_program: self.token_program.to_account_info(),
                system_program: self.system_program.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                cpi_accounts,
                seeds,
            );
            associated_token::create(cpi_ctx)?;
        }

        Ok(())
    }
}
