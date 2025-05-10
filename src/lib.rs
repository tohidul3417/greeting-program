use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    msg!("Greeting Program: process_instruction reporting for duty!");

    // Inspect the program_id that's running this instruction
    msg!("Program ID: {}", program_id);

    // Inspect the accounts passed to this instruction
    for (i, account) in accounts.iter().enumerate() {
        msg!("Account: {}, Pubkey: {}, Is _singer: {}, Is_writable: {}",
            i,
            account.key,
            account.is_signer,
            account.is_writable,
        );
    };

    // Inspect the instruction data passed to this instruction
    msg!("Instruction data length: {} bytes", instruction_data.len());
    if !instruction_data.is_empty() {
        msg!("First byte of instruction data: {}", instruction_data[0]);
    }

    Ok(())

}

// Basic tests (will not run on-chain, but good for local dev workflow)
#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_basic_invocation() {
        // Mock data for testing process_instruction locally
        let program_id = Pubkey::new_unique();
        let key = Pubkey::new_unique(); // A dummy account key
        let mut lamports = 0;
        let mut data = vec![0; 0]; // No data in this dummy account
        let owner = Pubkey::new_unique(); // Dummy owner

        let account = AccountInfo::new(
            &key,
            false, // is_signer
            true,  // is_writable
            &mut lamports,
            &mut data,
            &owner,
            false, // executable
            Epoch::default(),
        );
        let accounts = vec![account];
        let instruction_data: Vec<u8> = vec![1, 2, 3]; // Dummy instruction data

        // Call the function directly
        assert_eq!(
            process_instruction(&program_id, &accounts, &instruction_data),
            Ok(())
        );
    }
}