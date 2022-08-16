use crate::{
    handler::{
        create_obligation_tulip_levfarm, initialize_tulip_levfarm, transfer_lamports,
        CreateObligationTulipLevfarmAccounts, InitializeTulipLevfarmAccounts,
        TransferLamportsAccounts,
    },
    state, ChamberError, InitializeChamberStrategy,
};
use anchor_lang::prelude::*;

impl<'c, 'info> InitializeChamberStrategy<'info> {
    pub fn process(&mut self, remaining_accounts: &'c [AccountInfo<'info>]) -> Result<()> {
        // TODO: Extend protocols support
        match self.chamber.protocol_type {
            state::ProtocolType::Tulip => {
                // Fund PDA authority
                transfer_lamports(
                    Box::new(TransferLamportsAccounts {
                        from: &self.payer,
                        to: &self.authority,
                        system_program: &self.system_program,
                    }),
                    // TODO: Calculate exact lamports amount
                    (2610000 + 2999760 + 4565760) * 5,
                )?;

                // Create farm account and initialize 1st obligation
                // Initialize obligation LP ata
                initialize_tulip_levfarm(Box::new(InitializeTulipLevfarmAccounts {
                    chamber: &self.chamber,
                    chamber_authority: &self.authority.to_account_info(),
                    payer: &self.payer.to_account_info(),
                    global: &remaining_accounts[0],
                    chamber_farm: &remaining_accounts[1],
                    chamber_farm_obligation: &remaining_accounts[2], //  obligation for first position
                    chamber_obligation_vault: &remaining_accounts[4], //  obligation vault for first position
                    lending_market: &remaining_accounts[6],
                    leveraged_farm: &remaining_accounts[7],
                    raydium_lp_ata: &remaining_accounts[8], //  LP ata for first position
                    raydium_lp_mint: &remaining_accounts[10],
                    tulip_ata: &remaining_accounts[11], //  rewards ata for first position
                    tulip_mint: &remaining_accounts[13],
                    clock_sysvar: &self.clock_sysvar,
                    rent_sysvar: &self.rent_sysvar,
                    lending_program: &remaining_accounts[14],
                    levfarm_program: &remaining_accounts[15],
                    solfarm_vault_program: &remaining_accounts[16],
                    token_program: &self.token_program,
                    system_program: &self.system_program,
                }))?;

                // Create and initialize 2nd obligation
                // Initialize obligation LP ata
                create_obligation_tulip_levfarm(Box::new(CreateObligationTulipLevfarmAccounts {
                    chamber: &self.chamber,
                    chamber_authority: &self.authority,
                    chamber_farm: &remaining_accounts[1],
                    leveraged_farm: &remaining_accounts[7],
                    chamber_farm_obligation: &remaining_accounts[3], //  obligation for second position
                    lending_market: &remaining_accounts[6],
                    chamber_obligation_vault: &remaining_accounts[5], //  obligation vault for second position
                    raydium_lp_ata: &remaining_accounts[9],
                    raydium_lp_mint: &remaining_accounts[10], //  LP ata for second position
                    tulip_ata: &remaining_accounts[12],       //  rewards ata for second position
                    tulip_mint: &remaining_accounts[13],
                    payer: &self.payer,
                    clock_sysvar: &self.clock_sysvar,
                    rent_sysvar: &self.rent_sysvar,
                    lending_program: &remaining_accounts[14],
                    levfarm_program: &remaining_accounts[15],
                    token_program: &self.token_program,
                    system_program: &self.system_program,
                }))?;
            }
            _ => return Err(ChamberError::UnsupportedProtocol.into()),
        };

        Ok(())
    }
}
