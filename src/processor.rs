use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use crate::{instruction::MultiSigInstruction, state::MultiSig, error::MultiSigError};
use borsh::{BorshDeserialize, BorshSerialize};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    ) -> ProgramResult {
        let instruction = MultiSigInstruction::unpack(instruction_data)?;

        match instruction {
            MultiSigInstruction::Initialize { owners, threshold } => {
                Self::process_initialize(accounts, owners, threshold, program_id)
            },
            MultiSigInstruction::SubmitTransaction { transaction_id } => {
                Self::process_submit_transaction(accounts, transaction_id, program_id)
            },
            MultiSigInstruction::Approve { transaction_id } => {
                Self::process_approve(accounts, transaction_id, program_id)
            },
            MultiSigInstruction::Execute { transaction_id } => {
                Self::process_execute(accounts, transaction_id, program_id)
            },
        }
    }

    fn process_initialize(
        accounts: &[AccountInfo],
        owners: Vec<Pubkey>,
        threshold: u8,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let multisig_account = next_account_info(account_info_iter)?;

        if owners.len() < threshold as usize {
            msg!("Insufficient number of owners for the threshold.");
            return Err(ProgramError::InvalidInstructionData);
        }

        let multisig_data = MultiSig {
            owners,
            threshold,
            approvals: 0,
            executed: false,
        };

        multisig_data.serialize(&mut &mut multisig_account.data.borrow_mut()[..])?;
        Ok(())
    }

    fn process_submit_transaction(
        accounts: &[AccountInfo],
        transaction_id: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let multisig_account = next_account_info(account_info_iter)?;

        let mut multisig_data = MultiSig::try_from_slice(&multisig_account.data.borrow())?;
        
        if multisig_data.executed {
            msg!("Transaction already executed.");
            return Err(MultiSigError::AlreadyExecuted.into());
        }

        multisig_data.approvals = 0;
        multisig_data.executed = false;
        multisig_data.serialize(&mut &mut multisig_account.data.borrow_mut()[..])?;
        Ok(())
    }

    fn process_approve(
        accounts: &[AccountInfo],
        transaction_id: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let multisig_account = next_account_info(account_info_iter)?;
        let signer_account = next_account_info(account_info_iter)?;

        let mut multisig_data = MultiSig::try_from_slice(&multisig_account.data.borrow())?;

        if !multisig_data.owners.contains(signer_account.key) {
            msg!("Signer is not an owner.");
            return Err(MultiSigError::NotOwner.into());
        }

        multisig_data.approvals += 1;
        multisig_data.serialize(&mut &mut multisig_account.data.borrow_mut()[..])?;
        Ok(())
    }

    fn process_execute(
        accounts: &[AccountInfo],
        transaction_id: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let multisig_account = next_account_info(account_info_iter)?;

        let mut multisig_data = MultiSig::try_from_slice(&multisig_account.data.borrow())?;

        if multisig_data.approvals < multisig_data.threshold {
            msg!("Not enough approvals to execute transaction.");
            return Err(MultiSigError::InsufficientSigners.into());
        }

        multisig_data.executed = true;
        multisig_data.serialize(&mut &mut multisig_account.data.borrow_mut()[..])?;
        Ok(())
    }
}
