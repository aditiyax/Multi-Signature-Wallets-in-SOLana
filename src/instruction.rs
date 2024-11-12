use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum MultiSigInstruction {

    Initialize { owners: Vec<Pubkey>, threshold: u8 },

    SubmitTransaction { transaction_id: u64 },

    Approve { transaction_id: u64 },

    Execute { transaction_id: u64 },
}

impl MultiSigInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        match tag {
            0 => {
                let owners = Vec::<Pubkey>::deserialize(&mut &rest[..])?;
                let threshold = *rest.get(owners.len() * 32).ok_or(ProgramError::InvalidInstructionData)?;
                Ok(Self::Initialize { owners, threshold })
            },
            1 => {
                let transaction_id = u64::from_le_bytes(
                    rest.get(..8).ok_or(ProgramError::InvalidInstructionData)?.try_into().unwrap(),
                );
                Ok(Self::SubmitTransaction { transaction_id })
            },
            2 => {
                let transaction_id = u64::from_le_bytes(
                    rest.get(..8).ok_or(ProgramError::InvalidInstructionData)?.try_into().unwrap(),
                );
                Ok(Self::Approve { transaction_id })
            },
            3 => {
                let transaction_id = u64::from_le_bytes(
                    rest.get(..8).ok_or(ProgramError::InvalidInstructionData)?.try_into().unwrap(),
                );
                Ok(Self::Execute { transaction_id })
            },
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
