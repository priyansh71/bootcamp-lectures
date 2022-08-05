use std::borrow::Borrow;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum AccountTag {
    Uninitialized,
    Mint,
    TokenAccount,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Mint {
    pub tag: AccountTag,    // Distinguishes between mint and token account
    pub authority: Pubkey,  // Mint Authority
    pub supply: u64,        // Total supply of tokens
}

impl Mint {
    // Borrow data from mint account buffer by deserealizing it
    pub fn load_unchecked(ai: &AccountInfo) -> Result<Self, ProgramError> {
        Ok(Self::try_from_slice(&ai.data.borrow())?)
    }

    // Check if the account is a mint account
    fn validate(&self) -> ProgramResult {
        if self.tag != AccountTag::Mint {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    // Deserialize data from buffer and validate the account
    pub fn load(ai: &AccountInfo) -> Result<Self, ProgramError> {
        let mint = Self::try_from_slice(&ai.data.borrow())?;
        mint.validate()?;
        Ok(mint)
    }

    // Serialize changed data into buffer again
    pub fn save(&self, ai: &AccountInfo) -> ProgramResult {
        Ok(self.serialize(&mut *ai.data.borrow_mut())?)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TokenAccount {
    pub tag: AccountTag,        // Distinguishes between mint and token account
    pub owner: Pubkey,          // Owner of the token account
    pub mint: Pubkey,           // Mint type
    pub amount: u64,            // Amount of tokens in the account
}

impl TokenAccount {
    pub fn load_unchecked(ai: &AccountInfo) -> Result<Self, ProgramError> {
        Ok(Self::try_from_slice(&ai.data.borrow())?)
    }

    fn validate(&self) -> ProgramResult {
        if self.tag != AccountTag::TokenAccount {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    pub fn load(ai: &AccountInfo) -> Result<Self, ProgramError> {
        let account = Self::try_from_slice(&ai.data.borrow())?;
        account.validate()?;
        Ok(account)
    }

    pub fn save(&self, ai: &AccountInfo) -> ProgramResult {
        Ok(self.serialize(&mut *ai.data.borrow_mut())?)
    }
}
