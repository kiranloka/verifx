use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use anchor_lang::prelude::AccountDeserialize;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction, pubkey::Pubkey};
use verifx_program::{self, FileHash};

#[tokio::test]
async fn test_store_hash() {
    let program_id = verifx_program::id();
    let mut program = ProgramTest::new("verifx_program", program_id, None);

    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let file_name = "test.txt".to_string();
    let hash = [1u8; 32];

    let (pda, bump) = Pubkey::find_program_address(
        &[payer.pubkey().as_ref(), file_name.as_bytes()],
        &program_id,
    );

    let ix = solana_sdk::instruction::Instruction {
        program_id,
        accounts: verifx_program::accounts::StoreHash {
            file_hash: pda,
            user: payer.pubkey(),
            system_program: solana_program::system_program::ID,
        }
        .to_account_metas(None),
        data: verifx_program::instruction::StoreHash {
            file_name: file_name.clone(),
            hash,
        }
        .data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    let account = banks_client.get_account(pda).await.unwrap().unwrap();
    let file_hash = FileHash::try_deserialize(&mut &account.data[..]).unwrap();

    assert_eq!(file_hash.hash, hash);
}
