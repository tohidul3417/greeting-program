use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum GreetingInstruction {
    /// Creates a new greeting account (PDA) and sets an initial message.
    ///
    /// Accounts expected by this instruction:
    /// 0. `[signer, writable]` `payer_account`: The account paying for the new greeting account's rent
    ///                         and whose key will be used as a seed for the PDA. Becomes the authority.
    /// 1. `[writable]` `greeting_account_pda`: The PDA to be created and initialized.
    ///                         Its address is derived from `program_id`, `payer_account.key`, and potentially `name`.
    ///                         The client must pass the correct derived address here.
    /// 2. `[]` `system_program`: The Solana System Program, required for creating accounts.
    CreateGreeting {
        name: String,
        message: String,
    },

    /// Sets a new greeting message on an existing greeting account.
    /// 
    /// Accounts expected:
    /// 0. `[signer]` The authority of the greeting account.
    /// 1. `[writable]` The greeting account (PDA) whose message is to be changed.
   SetGreeting {
    message: String,
   },
    // We could add a `ResetGreeting` or `CloseGreetingAccount` later.
}


/// Structure of the data stored in a greeting account.
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct GreetingAccountState {
    // The Authority (public key) that is allowed to change the greeting.
    pub authority: Pubkey,

    // A name or title for the greeting.
    pub name: String,

    // The greeting message itself.
    pub message: String,

    // A counter for how many times the greeting has been updated (just for fun!).
    pub update_count: u32,
}


// Let's define some constraints, especially for Strings.
// Solana accounts have size limits. Unbounded strings are risky.
impl GreetingAccountState {
    // Max length for the 'name' field.
    pub const MAX_NAME_LENGTH: usize = 32;
    // Max length for the 'message' field.
    pub const MAX_MESSAGE_LENGTH: usize = 128;
    // Discriminator for account type, can be useful if the program manages multiple account types
    pub const ACCOUNT_DISCRIMINATOR: &'static str = "GREETING"; // Not strictly needed for borsh, but good practice for some patterns.
    // Calculate the maximum space needed for the account space.
    pub fn get_max_space_needed() -> usize {
    // Pubkey = 32 bytes
    // String length (u32 = 4 bytes) + max characters for name
    // String length (u32 = 4 bytes) + max characters for message
    // u32 = 4 bytes for update_count

    32 + // authority
    (4 + Self::MAX_NAME_LENGTH) + // name
    (4 + Self::MAX_MESSAGE_LENGTH) + // message
    4 // update_count
    }
}



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
        msg!("Account: {}, Pubkey: {}, Is _signer: {}, Is_writable: {}",
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


    // Attempt to deserialize the instruction data into our GreetingInstruction enum
    let instruction = GreetingInstruction::try_from_slice(instruction_data).map_err(|err| {
        msg!("Failed to deserialize instruction data: {}", err);
        ProgramError::InvalidInstructionData
    })?;

    // Now we can match on the specific instruction variant
    match instruction {
        GreetingInstruction::CreateGreeting { name, message } => {
            msg!("Instruction: CreateGreeting");
            msg!("Name: {}", name);
            msg!("Message: {}", message);
            // Here we would add logic to:
            // 1. Validate name and message lengths against MAX_NAME_LENGTH and MAX_MESSAGE_LENGTH.
            // 2. Process the accounts to create and initialize the greeting account.
            // We'll do this in the next step.
        }
        GreetingInstruction::SetGreeting { message } => {
            msg!("Instruction: SetGreeting");
            msg!("New Message: {}", message);
            // Here we would add logic to:
            // 1. Validate message length.
            // 2. Process accounts to ensure the signer is the authority.
            // 3. Update the message in the greeting account.
            // We'll do this in the next step.
        }
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