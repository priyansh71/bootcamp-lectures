use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum TokenInstruction {
    InitializeMint,
    InitializeTokenAccount,
    Mint { amount: u64 },   // Increase Supply
    Burn { amount: u64 },   // Decrease Supply
    Transfer { amount: u64 }, // Transfer tokens to another account
}
