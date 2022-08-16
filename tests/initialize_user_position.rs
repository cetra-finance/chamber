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

    test_chamber_tulip
        .initialize_chamber(&mut test_context, &payer)
        .await
        .unwrap();

    let test_user = TestUser::new();

    // Fund wallet with 1 SOL
    test_user
        .fund(&mut test_context, &payer, 1000000000)
        .await
        .unwrap();

    // SOL with 100$ (~=2.5)
    test_user
        .create_ata(&mut test_context, &test_utils::wsol_mint::id(), 2500000000)
        .await
        .unwrap();

    // USDC with 100$
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

    let user_position = test_user
        .load_user_position(&mut test_context, &test_chamber_tulip)
        .await
        .unwrap();

    assert_eq!(user_position.base_amount, 2500000000);
    assert_eq!(user_position.quote_amount, 100000000);
    assert_eq!(user_position.chamber, test_chamber_tulip.pubkey);
}
