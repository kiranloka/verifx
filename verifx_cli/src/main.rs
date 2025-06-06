use clap::{App,Arg,SubCommand}
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::{AccoundMeta,Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file,Keypair,Signer};
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;
use sha2::{Digest,Sha256}
use skim::prelude::*;
use walkdir::WalkDir;
use colored::*;
use indicatif::ProgressBar;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use borsh::BorshSerialize;

const PROGRAM_ID:&str="ProgramId";
const ASCII_ART:&str=r#"




        _   _           _________   __       
 ______| | | |         (_)  ___\ \ / /______ 
|______| | | | ___ _ __ _| |_   \ V /|______|
 ______| | | |/ _ \ '__| |  _|  /   \ ______ 
|______\ \_/ /  __/ |  | | |   / /^\ \______|
        \___/ \___|_|  |_\_|   \/   \/       
                                             
                                             
"#;

#[tokio::main]
async fn main(){
    println!("{}",ASCII_ART.bright_cyan());

    let matches=App::new("veriFX")
        .version("0.1.0")
        .about("File integrity verifier using Solana Blockchain")
        .arg(Arg::with_name("keypair"))
        .short("k")
        .long("keypair")
        .value_name("FILE")
        .help("Path to Solana keypair file")
        .required(true)
        .takes_value(true)
        .subcommmand(
            SubCommand::with_name("store")
            .about("Store a file's hash on the blockchain")
            .arg(Arg::with_name("file")
            .help("Path to file is optional , uses interactive selection if omitted")
            .required(false)),
        )
        .get_matches();


    let keypair_matches=matches.value_of("keypair").unwrap();
    let keypair=read_keypair_file(keypair_file).expect("Failed to read the keypair file");
    let client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    match matches.subcommand(){
        ("store",Some(sub_m))=>{
            let file_path=get_file_path(sub_m.value_of("file")).expect("Failed to get the file path");
            store_hash(&client,&keypair,&file_path).await.expect("Failed to store hash");

        }
        ("verify",Some(sub_m))=>{
            let file_path=get_file_path(sub_m.value_of("file")).expect("Failed to get the file path");
            verify_hash(&client,&keypair,&file_path).await.expect("Failed to verify the file");

        }
        _=>println!("{}","Invalid command. Use 'store' or 'verify' .".red())
    }
}


fn get_file_path(arg:Option<&str>)->Result<String,Box<dyn std::error::Error>>{
    if let Some(path)=arg{
        Ok(path.to_string())
    }else{
        select_file().ok_or("No file selected".into())
    }
}

fn select_file()->Option<String>{
    let files:Vec<String>=WalkDir::new(".")
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().is_file())
    .map(|e| e.path().to_string_lossy().into_owned())
    .collect();


    let options=SkimOptionBuilder::default()
    .height(Some("50%"))
    .multi(false)
    .build()
    .unwrap();

    let item_reader=Skim::run_with(&options,Some(items))
    .map(|out| out.selected_items)
    .unwrap_or_else(Vec::new);

    selected.first().map(|item| item.output().to_string())
}


fn compute_hash(file_path:&str)->Result<[u8;32],Box<dyn std::error:Error>>{
    let mut=File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer=[0;1024];

    loop{
        let bytes_read=file.read(&mut buffer)?;
        if bytes_read==0{
            break;
        }

        hasher.update(&buffer[..buffer_read]);
    }
    Ok(hasher.finalize().into())
}


async fn store_hash(
    client:&RpcClient,
    keypair:&Keypair,
    filePath:&str,
)->Result<(),Box<dyn std::error::Error>>{
    let hash=compute_hash(file_path)?;
    let file_name=Path::new(file_path).file_name().unwrap().to_str().unwrap();
    let program_id=Pubkey::new_from_array(
        bs58::decode(PROGRAM_ID)
        .into_vec()?
        .try_into()
        .unwrap(),
    );

    let (pda,_bump)=Pubkey::find_program_address(
        &[keypair.pubkey().as_ref(),file_name.as_bytes()],
        &program_id,
    );


    let discriminator=&sha256::digest(b"global:store_hash")[..8];
    let file_name_bytes=file_name.as_bytes();
    let file_name_len=(file_name_bytes.len() as u32).to_le_bytes();
    let mut instruction_data=Vec::new();
    instruction_data.extend_from_slice(discriminator);
    instruction_data.extend_from_slice(&file_name_len);
    instruction_data.extend_from_slice(file_name_bytes);
    instruction_data.extend_from_slice(&hash);


    let instructions=Instruction{
        program_id,
        accounts:vec![
            AccountMeta::new(pda,false),
            AccoundMeta::new(keypair.pubkey(),true),
            AccountMeta::new_readonly(system_program::id(),false),
        ],

        data;instruction_data;
    };


    let pb=ProgressBar::new_spinner();
    pb.set_message("Storing hash on solana blockchain...");
    pb.enable_steady_tick(100);


    let recent_blockhash=client.get_latest_blockhash()?;
    let transaction=Transaction::new_signed_with_payer(
        &[instruction],
        Some(&keypair.pubkey()),
        &[keypair],
        recent_blockhash,
    );

    let signature=client.send_and_confirm_transaction(&transaction)?;

    pb.finish_with_message("Hash stored on Solana successfully!");
    println!("Transaction signed : {}",signature);
    OK(())
}

async fn verify_hash(
    client:&RpcClient,
    keypair:&Keypair,
    file_path:&str,
)->Result<(),Box<dyn std::error::Error>>{
    let current_hash=compute_hash(file_path)?;
    let file_name=Path::new(file_path).file_name().unwrap().to_str().unwrap();
    let program_id=Pubkey::new_from_array(
        bs58::decode(PROGRAM_ID)
        .into_vec()?
        .try_into()
        .unwrap(),
    );

    let(pda,_bump)=PubKey::find_program_address(
        &[keypair.pubkey().as_ref(),file_name.as_bytes()],
        &program_id,
    );


    let pb=ProgressBar::new_spinner();
    pb.set_message("Fetching hash from Solana Blockchain...");
    pb.enable_steady_tick(100);
    


    let account=client.get_account(&pda).ok();
    pb.finish();


    if Let Some(account)=account{
        let data=account.data;
        if data.len()>=40{
            let stored_hash:[u8;32]=data[8..40].try_into().unwrap();
            if stored_hash==current_hash{
                println!("{}","File Integrity verified!".green());

            }else{
                println!("{}","File has been modified!".red())
            }
        }else{
            println!("{}","Invalid accound data".yellow());

        }
    }else{
        println!("{}","No hash found for this file!")
    }
    Ok(())
}



