use std::io::Error;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::CustomError;
use crate::instruction::HelloWorldInstruction;
use crate::Notebook;
use crate::state::CONTEXT_LENGTH_LIMIT;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey, // Public key of the account the hello world program was loaded into
        accounts: &[AccountInfo], // The account to say hello to
        _instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Hello World Rust program entrypoint");

        // 账户权限校验
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;
        if account.owner != program_id {
            msg!("Account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        // 解析account数据
        let mut notebook_account = Notebook::try_from_slice(&account.data.borrow_mut())?;

        // 解析指令
        msg!("_instruction_data: {:?}", _instruction_data);
        let instruction = HelloWorldInstruction::try_from_slice(_instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData).unwrap();

        // Controller逻辑
        let result = match &instruction {
            HelloWorldInstruction::Init { data, owner } => {
                init(&mut notebook_account, data, owner)
            }
            HelloWorldInstruction::Read => {
                read(&notebook_account)
            }
            HelloWorldInstruction::Write { data, owner } => {
                write(&mut notebook_account, data, owner)
            }
        };

        // store data
        // let result = Ok(());
        // notebook_account.data = "load".to_string();

        match notebook_account.serialize(&mut *account.data.borrow_mut()) {
            Ok(_) => { msg!("notebook_account 序列化成功") }
            Err(e) => {
                msg!("notebook_account 序列化失败") ;
                msg!("{}", e);
            }
        }

        result
    }
}

#[inline]
fn init(
    notebook_account: &mut Notebook, data: &String, owner: &String,
) -> ProgramResult {
    msg!("Init Start");
    match is_data_length_ok(data) {
        true => {
            match notebook_account.is_init {
                true => { Err(CustomError::AuthorizationErrorDoubleInit.into()) }
                false => {
                    notebook_account.owner = owner.clone();
                    notebook_account.is_init = true;
                    notebook_account.data = data.clone();
                    msg!("Init Finish: {:?}", notebook_account);
                    Ok(())
                }
            }
        }
        false => { Err(CustomError::LengthLimitError.into()) }
    }
}

#[inline]
fn write(
    notebook_account: &mut Notebook, data: &String, owner: &String,
) -> ProgramResult {
    msg!("Write Start");
    match is_data_length_ok(data) {
        true => {
            match is_owner_ok(owner, &notebook_account) {
                true => {
                    notebook_account.data = data.clone();
                    msg!("Write Finish: {:?}", notebook_account);
                    Ok(())
                }
                false => { Err(CustomError::AuthorizationErrorNoWritePermission.into()) }
            }
        }
        false => Err(CustomError::LengthLimitError.into())
    }
}

#[inline]
fn read(notebook_account: &Notebook) -> ProgramResult {
    msg!("Read Start");
    let data: String = notebook_account.data.clone();
    msg!("Read Finish: {}", data);
    Ok(())
}

/// 检查data长度是否越界
#[inline]
fn is_data_length_ok(data: &String) -> bool {
    data.len() as u32 <= CONTEXT_LENGTH_LIMIT
}

/// 写入权限校验
#[inline]
fn is_owner_ok(owner: &String, notebook_account: &Notebook) -> bool {
    owner.eq(&notebook_account.owner)
}