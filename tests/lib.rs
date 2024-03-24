use borsh::{BorshDeserialize, to_vec};
use solana_program_test::{BanksClientError, processor, ProgramTest, tokio};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};

use helloworld::{Notebook, process_instruction};
use helloworld::instruction::HelloWorldInstruction;

#[tokio::test]
async fn test_helloworld() {
    let program_id = Pubkey::new_unique();
    let notebook_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "SolanaHelloWorld", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );

    // 生成PDA(Program Derived Addresses)
    // 将账户Account与地址 notebook_pubkey 进行绑定
    program_test.add_account(
        notebook_pubkey,
        Account {
            lamports: 5,
            // data: vec![0_u8; mem::size_of::<u32>()], // 初始化数据
            data: to_vec::<Notebook>(&Notebook{
                data: "data0".to_string(),
                owner: "ownerx".to_string(),
                is_init: false,
            }).unwrap(), // 初始化数据
            owner: program_id,
            ..Account::default()
        },
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // get account data
    let mut notebook_account = banks_client
        .get_account(notebook_pubkey)
        .await
        .expect("get_account")
        .expect("notebook_account not found");

    // Init
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            program_id,
            &to_vec(&HelloWorldInstruction::Init {
                data: "data1".to_string(),
                owner: "owner1".to_string()
            }).unwrap(),
            vec![AccountMeta::new(notebook_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account
    notebook_account = banks_client
        .get_account(notebook_pubkey)
        .await
        .expect("get_account")
        .expect("notebook_account not found");
    assert_eq!(
        // Notebook::try_from_slice(&notebook_account.data).unwrap(),Notebook::try_from_slice(&notebook_account.data).unwrap(),
        Notebook::deserialize(&mut &notebook_account.data[..]).unwrap(),
        Notebook {
            data: "data1".to_string(),
            owner: "owner1".to_string(),
            is_init: true,
        }
    );

    // Read1
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            program_id,
            &to_vec(&HelloWorldInstruction::Read).unwrap(),
            vec![AccountMeta::new(notebook_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account
    notebook_account = banks_client
        .get_account(notebook_pubkey)
        .await
        .expect("get_account")
        .expect("notebook_account not found");
    assert_eq!(
        // Notebook::try_from_slice(&notebook_account.data).unwrap(),Notebook::try_from_slice(&notebook_account.data).unwrap(),
        Notebook::deserialize(&mut &notebook_account.data[..]).unwrap(),
        Notebook {
            data: "data1".to_string(),
            owner: "owner1".to_string(),
            is_init: true,
        }
    );

    // Write1
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            program_id,
            &to_vec(&HelloWorldInstruction::Write {
                data: "data2".to_string(),
                owner: "owner1".to_string()
            }).unwrap(),
            vec![AccountMeta::new(notebook_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account
    notebook_account = banks_client
        .get_account(notebook_pubkey)
        .await
        .expect("get_account")
        .expect("notebook_account not found");
    assert_eq!(
        // Notebook::try_from_slice(&notebook_account.data).unwrap(),Notebook::try_from_slice(&notebook_account.data).unwrap(),
        Notebook::deserialize(&mut &notebook_account.data[..]).unwrap(),
        Notebook {
            data: "data2".to_string(),
            owner: "owner1".to_string(),
            is_init: true,
        }
    );

    // Read2
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            program_id,
            &to_vec(&HelloWorldInstruction::Read).unwrap(),
            vec![AccountMeta::new(notebook_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account
    notebook_account = banks_client
        .get_account(notebook_pubkey)
        .await
        .expect("get_account")
        .expect("notebook_account not found");
    assert_eq!(
        // Notebook::try_from_slice(&notebook_account.data).unwrap(),Notebook::try_from_slice(&notebook_account.data).unwrap(),
        Notebook::deserialize(&mut &notebook_account.data[..]).unwrap(),
        Notebook {
            data: "data2".to_string(),
            owner: "owner1".to_string(),
            is_init: true,
        }
    );

    // Write2
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            program_id,
            &to_vec(&HelloWorldInstruction::Write {
                data: "data3".to_string(),
                owner: "owner2".to_string()
            }).unwrap(),
            vec![AccountMeta::new(notebook_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    match banks_client.process_transaction(transaction).await {
        Ok(_) => {}
        Err(_) => {}
    }

    // Verify account
    notebook_account = banks_client
        .get_account(notebook_pubkey)
        .await
        .expect("get_account")
        .expect("notebook_account not found");
    assert_eq!(
        // Notebook::try_from_slice(&notebook_account.data).unwrap(),Notebook::try_from_slice(&notebook_account.data).unwrap(),
        Notebook::deserialize(&mut &notebook_account.data[..]).unwrap(),
        Notebook {
            data: "data2".to_string(),
            owner: "owner1".to_string(),
            is_init: true,
        }
    );
}
