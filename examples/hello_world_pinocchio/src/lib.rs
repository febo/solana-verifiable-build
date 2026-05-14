use pinocchio::{error::ProgramError, program_entrypoint, AccountView, Address, ProgramResult};
use solana_program_log::log;

program_entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    log("Hello, world!");
    if program_id != &Address::new_from_array([0; 32]) {
        Ok(())
    } else {
        Err(ProgramError::IncorrectProgramId)
    }
}
