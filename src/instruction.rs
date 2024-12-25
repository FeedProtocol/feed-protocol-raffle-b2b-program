use crate::{error::RaffleProgramError::InvalidInstruction, state::{Fee, InitPda, Raffle, Winner}};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(Debug, PartialEq)]
pub enum RaffleProgramInstruction {
    InitRaffle{init_raffle:Raffle},
    PublishWinner{winner:Winner},
    InitCounter,
    InitFee{data:InitPda},
    InitConfig,
    SetConfig,
    UpdateFee{data:Fee},
    CollectFee,
}

impl RaffleProgramInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {

    let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
    Ok(match tag {
      0 => Self::InitRaffle{
        init_raffle:Raffle::try_from_slice(&rest)?
      },
      1 => Self::PublishWinner{
        winner:Winner::try_from_slice(&rest)?
      },
      2 => Self::InitCounter,
      3 => Self::InitFee{
        data:InitPda::try_from_slice(&rest)?
      },
      4 => Self::InitConfig,
      5 => Self::SetConfig,
      6 => Self::UpdateFee{
        data:Fee::try_from_slice(&rest)?
      },
      7 => Self::CollectFee,
      

      _ => return Err(InvalidInstruction.into()),
    })
  }
}
