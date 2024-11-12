use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum MultiSigError {
    #[error("Insufficient signers")]
    InsufficientSigners,
    #[error("Transaction already executed")]
    AlreadyExecuted,
    #[error("Owner not recognized")]
    NotOwner,
}

impl From<MultiSigError> for ProgramError {
    fn from(e: MultiSigError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
