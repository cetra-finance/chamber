mod test_state;
mod test_utils;

use cetra_chamber::state::ProtocolType;
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

    let test_chamber_tulip_state = test_chamber_tulip.load(&mut test_context).await.unwrap();
    assert_eq!(test_chamber_tulip_state.protocol_type, ProtocolType::Tulip);
}
