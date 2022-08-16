use solana_sdk::signature::Keypair;

pub mod usdc_mint {
    use anchor_lang::declare_id;

    declare_id!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
}

pub mod serum_program {
    use anchor_lang::declare_id;

    declare_id!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");
}

pub mod raydium_raysrm_farm {
    use anchor_lang::declare_id;

    declare_id!("GUzaohfNuFbBqQTnPgPSNciv3aUvriXYjQduRE3ZkqFw");
}

pub mod usdt_mint {
    use anchor_lang::declare_id;

    declare_id!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
}

pub mod wsol_mint {
    pub use anchor_spl::token::spl_token::native_mint::id;
}

pub mod tulip_mint {
    use anchor_lang::declare_id;

    declare_id!("TuLipcqtGVXP9XR62wM8WWCm6a9vhLs7T1uoWBk6FDs");
}

pub fn clone_keypair(keypair: &Keypair) -> Keypair {
    Keypair::from_bytes(&keypair.to_bytes()).unwrap()
}
