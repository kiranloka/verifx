use anchor_lang::prelude::*;
use verifix_program::{FileHash,StoreHash};
use solana_program_test::*;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;


#[tokio::test]
async fn test_store_hash(){
    let mut program = ProgramTest::default();
    program.add_program("verifx_program",verifx_program::id(),None);
    let (mut banks_client,payer,recent_blockhash)=program.start().await;
     
    let file_name="test.txt".to_string();
    let hash=[1u8,32];
    let (pda,bump)=Pubkey::find_program_address(
        &[payer.pubkey().as_ref(),file_name.as_bytes()],
        &verifx_program::id(),
    );


    let tx=solana_sdk::transaction::Transaction::new_signed_with_payer(
        $[verifx_program::instruction::store_hash(
            &payer.pubkey(),
            &file_name,
            hash,
            pda,
            bump,
        )],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(tx).await.unwrap();

    let account=banks_client.get_account(pda).await.unwrap().unwrap();
    let file_hash:FileHash=FileHash::try_deserialize(&mut &account.data[8..]).unwrap();
    assert_eq!(file_hash,hash);
}