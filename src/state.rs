use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]//153
pub struct Raffle{
    pub participants_hash:[u8;32],
    pub initializer:[u8;32],
    pub raffle_no:u64,
    pub number_of_participants:u64,
    pub winner_no:u64,
    pub winner_wallet:[u8;32],
    pub raffle_name:[u8;32],
    pub is_published:u8,
}



#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]//9
pub struct RaffleCounter{
    pub initialized:u8,
    pub number_of_raffles:u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]//9
pub struct Config{
    pub authority_1:[u8;32],
    pub authority_2:[u8;32],
    pub authority_3:[u8;32],
    pub authority_4:[u8;32],
}


#[derive(BorshDeserialize)]
pub struct RandomNumber{
  pub random_number:u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct InitPda{
    pub bump:u8,
    pub lamports:u64,
    pub no:u8,
}


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Fee{
    pub fee:u64
}
