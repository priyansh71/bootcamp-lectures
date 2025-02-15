use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::CounterInstruction;
use crate::state::Counter;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CounterInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            CounterInstruction::Increment => {
                msg!("Instruction: Increment");
                let accounts_iter = &mut accounts.iter();
                let counter_ai = next_account_info(accounts_iter)?;
                let mut counter = Counter::try_from_slice(&counter_ai.try_borrow_mut_data()?)?;
                counter.count += 1;
                msg!("Updated Count : {}", counter.count);
                counter.serialize(&mut *counter_ai.data.borrow_mut())?;
            }
            CounterInstruction::Decrement => {
                msg!("Instruction: Decrement");
                let accounts_iter = &mut accounts.iter();
                let counter_ai = next_account_info(accounts_iter)?;                                 // Returns next account in the iterator
                let mut counter = Counter::try_from_slice(&counter_ai.try_borrow_mut_data()?)?;   // Deserialize the account data
                counter.count = counter.count.saturating_sub(1);      
                msg!("Updated Count : {}", counter.count);
                counter.serialize(&mut *counter_ai.data.borrow_mut())?;                           // Serialize the counter back to the account data
            }
        }
        Ok(())
    }
}
