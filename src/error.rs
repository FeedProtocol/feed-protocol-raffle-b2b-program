use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum RaffleProgramError {
  /// Invalid Instruction
  #[error("Invalid Instruction")]//0
  InvalidInstruction,

  #[error("arithmetic error")]//1
  ArithmeticError,

  #[error("counter account is invalid")]//2
  InvalidCounter,
  
  #[error("invalid authhority account")]//3
  InvalidAuth,

  #[error("raffle account is invalid")]//4
  InvalidRaffle,

  #[error("initializer account is not signer")]//5
  InitializerNotSigner,

  #[error("invalid config account")]//6
  InvalidConfig,
  
  #[error("authority not signer ")]//7
  NotSignerAuth,

  #[error("initializer account is not valid ")]//8
  InvalidInitializer,

  #[error("invalid fee")]//9
  InvalidFee

}

impl From<RaffleProgramError> for ProgramError {
  fn from(e: RaffleProgramError) -> Self {
    ProgramError::Custom(e as u32)
  }
}
