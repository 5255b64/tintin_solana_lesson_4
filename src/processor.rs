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
        let mut notebook_account = Notebook::try_from_slice(&account.data.borrow())?;

        // 解析指令
        let tmp1 = HelloWorldInstruction::try_from_slice(_instruction_data);
        msg!("tmp1: {:?}", &tmp1);
        let tmp2 = tmp1.map_err(|_| ProgramError::InvalidInstructionData);
        msg!("tmp2: {:?}", &tmp2);
        let instruction = tmp2.unwrap();
        msg!("instruction: {:?}", &instruction);
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

        // // Increment and store the number of times the account has been greeted
        // let mut notebook_account = NotebookAccount::try_from_slice(&account.data.borrow())?;
        // notebook_account.counter += 1;
        notebook_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

        result
    }
}

#[inline]
fn init(
    notebook_account: &mut Notebook, data: &String, owner: &String,
) -> ProgramResult {
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
    notebook_account:&mut Notebook, data: &String, owner: &String,
) -> ProgramResult {
    match is_data_length_ok(data) {
        true => {
            match is_owner_ok(owner, &notebook_account){
                true => {
                    notebook_account.data=data.clone();
                    msg!("Write Finish: {:?}", notebook_account);
                    Ok(())
                }
                false => {Err(CustomError::AuthorizationErrorNoWritePermission.into())}
            }
        }
        false => Err(CustomError::LengthLimitError.into())
    }
}

#[inline]
fn read(notebook_account: &Notebook) -> ProgramResult {
    let data:String = notebook_account.data.clone();
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