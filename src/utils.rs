use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
};

pub fn is_signer(account_info: &AccountInfo, pubkey: &Pubkey) -> bool {
    account_info.is_signer && account_info.key == pubkey
}
