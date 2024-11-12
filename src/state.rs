use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MultiSig {
    pub owners: Vec<Pubkey>,
    pub threshold: u8,
    pub approvals: u8,
    pub executed: bool,
}
