mod test_state;
mod test_utils;

use cetra_program_test::{solana_program_test::*, *};
use test_state::*;
use test_utils::*;

#[tokio::test(flavor = "multi_thread")]
async fn success() {
    let rpc_account_loader = RpcAccountLoader::default();
    let mut program_test_loader = ProgramTestLoader::default();

    program_test_loader
        .program_test
        .add_program("cetra_chamber", cetra_chamber::id(), None);
    program_test_loader.load().unwrap();

    let mut test_context = program_test_loader
        .start_with_context(Box::new(rpc_account_loader))
        .await;

    let payer = clone_keypair(&test_context.context.payer);
    let test_chamber_tulip = TestChamberTulip::new_sol_usdc_raydium();

    let pyth_price_account = test_context
        .get_account(&test_chamber_tulip.farm_config.coin_price_account)
        .await
        .unwrap()
        .unwrap();
    let pyth_price =
        tulipv2_sdk_common::pyth::load::<tulipv2_sdk_common::pyth::Price>(&pyth_price_account.data)
            .unwrap();

    test_context
        .context
        .warp_to_slot(pyth_price.valid_slot)
        .unwrap();

    test_chamber_tulip
        .initialize_chamber(&mut test_context, &payer)
        .await
        .unwrap();

    test_chamber_tulip
        .initialize_chamber_strategy(&mut test_context, &payer)
        .await
        .unwrap();

    let test_user = TestUser::new();

    // Fund wallet with 1 SOL
    test_user
        .fund(&mut test_context, &payer, 1000000000)
        .await
        .unwrap();

    // SOL with 100$ (2.5)
    // User associated token account with SOL
    test_user
        .create_ata(&mut test_context, &test_utils::wsol_mint::id(), 2500000000)
        .await
        .unwrap();

    // USDC with 100$
    // User associated token account with USDC
    test_user
        .create_ata(&mut test_context, &test_utils::usdc_mint::id(), 100000000)
        .await
        .unwrap();

    test_user
        .initialize_user_position(
            &mut test_context,
            &test_chamber_tulip,
            2500000000,
            100000000,
        )
        .await
        .unwrap();

    test_chamber_tulip
        .deposit_chamber(&mut test_context, &test_user, 2500000000, 100000000)
        .await
        .unwrap();

    test_chamber_tulip
        .settle_chamber_position(&mut test_context, &test_user.wallet)
        .await
        .unwrap();

    test_chamber_tulip
        .settle_chamber_position2(&mut test_context, &test_user.wallet)
        .await
        .unwrap();
}
