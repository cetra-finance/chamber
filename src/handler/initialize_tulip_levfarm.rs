use crate::{state, utils};
use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::{associated_token, token::Token};
use tulipv2_sdk_levfarm::instructions::create_user_farm::{create_user_farm, CreateUserFarm};

pub struct InitializeTulipLevfarmAccounts<'c, 'info> {
    pub chamber: &'c Account<'info, state::Chamber>,
    pub chamber_authority: &'c AccountInfo<'info>,
    pub payer: &'c AccountInfo<'info>,
    pub global: &'c AccountInfo<'info>,
    pub chamber_farm: &'c AccountInfo<'info>,
    pub chamber_farm_obligation: &'c AccountInfo<'info>,
    pub chamber_obligation_vault: &'c AccountInfo<'info>,
    pub lending_market: &'c AccountInfo<'info>,
    pub leveraged_farm: &'c AccountInfo<'info>,
    pub raydium_lp_ata: &'c AccountInfo<'info>,
    pub raydium_lp_mint: &'c AccountInfo<'info>,
    pub tulip_ata: &'c AccountInfo<'info>,
    pub tulip_mint: &'c AccountInfo<'info>,
    pub clock_sysvar: &'c Sysvar<'info, Clock>,
    pub rent_sysvar: &'c Sysvar<'info, Rent>,
    pub lending_program: &'c AccountInfo<'info>,
    #[allow(unused)]
    pub levfarm_program: &'c AccountInfo<'info>,
    pub solfarm_vault_program: &'c AccountInfo<'info>,
    pub token_program: &'c Program<'info, Token>,
    pub system_program: &'c Program<'info, System>,
}

#[inline(always)]
pub fn initialize_tulip_levfarm<'c, 'info>(
    accounts: Box<InitializeTulipLevfarmAccounts<'c, 'info>>,
) -> Result<()> {
    // 1. Create LP associated token account
    {
        let cpi_accounts = associated_token::Create {
            payer: accounts.payer.to_account_info(),
            associated_token: accounts.raydium_lp_ata.to_account_info(),
            authority: accounts.chamber_obligation_vault.to_account_info(),
            mint: accounts.raydium_lp_mint.to_account_info(),
            rent: accounts.rent_sysvar.to_account_info(),
            token_program: accounts.token_program.to_account_info(),
            system_program: accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(accounts.token_program.to_account_info(), cpi_accounts);
        associated_token::create(cpi_ctx)?;
    }

    // TODO: 2. Create Tulip associated token account
    /* {
        let cpi_accounts = associated_token::Create {
            payer: accounts.payer.to_account_info(),
            associated_token: accounts.tulip_ata.to_account_info(),
            authority: accounts.chamber_obligation_vault.to_account_info(),
            mint: accounts.tulip_mint.to_account_info(),
            rent: accounts.rent_sysvar.to_account_info(),
            token_program: accounts.token_program.to_account_info(),
            system_program: accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(accounts.token_program.to_account_info(), cpi_accounts);
        associated_token::create(cpi_ctx)?;
    } */

    // 3. Initialize Tulip specific accounts
    {
        let ix = create_user_farm(
            CreateUserFarm {
                authority: accounts.chamber_authority.key(),
                user_farm: accounts.chamber_farm.key(),
                user_farm_obligation: accounts.chamber_farm_obligation.key(),
                lending_market: accounts.lending_market.key(),
                global: accounts.global.key(),
                leveraged_farm: accounts.leveraged_farm.key(),
                clock: accounts.clock_sysvar.key(),
                rent: accounts.rent_sysvar.key(),
                system_program: accounts.system_program.key(),
                lending_program: accounts.lending_program.key(),
                token_program: accounts.token_program.key(),
                obligation_vault_address: accounts.chamber_obligation_vault.key(),
            },
            accounts.solfarm_vault_program.key(),
        )
        .unwrap();

        invoke_signed(
            &ix,
            &Box::new(vec![
                accounts.chamber_authority.to_account_info(),
                accounts.chamber_farm.to_account_info(),
                accounts.chamber_farm_obligation.to_account_info(),
                accounts.lending_market.to_account_info(),
                accounts.global.to_account_info(),
                accounts.leveraged_farm.to_account_info(),
                accounts.clock_sysvar.to_account_info(),
                accounts.rent_sysvar.to_account_info(),
                accounts.system_program.to_account_info(),
                accounts.lending_program.to_account_info(),
                accounts.token_program.to_account_info(),
                accounts.chamber_obligation_vault.to_account_info(),
            ]),
            &[&[
                utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                accounts.chamber.key().as_ref(),
                &[accounts.chamber.authority_bump],
            ]],
        )?;
    }

    Ok(())
}
