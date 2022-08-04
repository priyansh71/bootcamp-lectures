use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::error::EchoError;
use crate::instruction::EchoInstruction;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EchoInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            EchoInstruction::Echo { data} => {
                msg!("Instruction: Echo");
                let accounts_iter = &mut _accounts.iter();
                let echo_buffer = next_account_info(accounts_iter)?;
                if !echo_buffer.is_writable {
                    return Err(ProgramError::AccountAlreadyInitialized);
                }
                let buffer_data = &mut (*echo_buffer.data).borrow_mut();
                msg!("{:?}", buffer_data);
                if !buffer_data.is_empty() {
                    return Err(ProgramError::AccountDataTooSmall);
                }
                let bytes_to_copy = buffer_data.len();
                if bytes_to_copy < data.len() {
                    for index in 0..bytes_to_copy {
                        buffer_data[index] = data[index]
                    }
                }
                msg!("{:?}", buffer_data);
                Ok(())
            }
            EchoInstruction::InitializeAuthorizedEcho {
                buffer_seed: _,
                buffer_size: _,
            } => {
                msg!("Instruction: InitializeAuthorizedEcho");
                Err(EchoError::NotImplemented.into())
            }
            EchoInstruction::AuthorizedEcho { data: _ } => {
                msg!("Instruction: AuthorizedEcho");
                Err(EchoError::NotImplemented.into())
            }
            EchoInstruction::InitializeVendingMachineEcho {
                price: _,
                buffer_size: _,
            } => {
                msg!("Instruction: InitializeVendingMachineEcho");
                Err(EchoError::NotImplemented.into())
            }
            EchoInstruction::VendingMachineEcho { data: _ } => {
                msg!("Instruction: VendingMachineEcho");
                Err(EchoError::NotImplemented.into())
            }
        }
    }
}
