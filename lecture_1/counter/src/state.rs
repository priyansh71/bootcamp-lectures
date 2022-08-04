use borsh::{BorshDeserialize, BorshSerialize};

// Stores the actual counter value in a Counter struct.
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Counter {
    // TODO
    pub count: u64,
}
