use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
  pubkey::Pubkey,
  account_info::{AccountInfo, next_account_info},
  entrypoint::ProgramResult,
  program_error::ProgramError,
  msg
};

use crate::instruction::EchoInstruction;
use crate::state::Echo;

pub struct Processor {}

impl Processor {
  pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
  ) -> ProgramResult {
    let instruction = EchoInstruction::try_from_slice(instruction_data)
      .map_err(|_| ProgramError::InvalidInstructionData)?;

    let accounts_iter = &mut accounts.iter();

    match instruction {
      EchoInstruction::Echo { data } => {
        msg!("Instruction: Echo");
        let echo_buffer = next_account_info(accounts_iter)?;

        let mut echo = Echo::try_from_slice(&echo_buffer.data.borrow())?;
        echo.data = data;
        echo.serialize(&mut *echo_buffer.data.borrow_mut())?;
      }
      _ => {}
    }
    Ok(())
  }
}
