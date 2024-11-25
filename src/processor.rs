use crate::{instruction::RaffleProgramInstruction, state::{ Config, Fee, InitPda, Raffle, RaffleCounter, RandomNumber}};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, instruction::{AccountMeta, Instruction}, msg, program::{get_return_data, invoke, invoke_signed}, pubkey::Pubkey, rent::Rent, system_instruction
};
use crate::error::RaffleProgramError::{InvalidCounter, ArithmeticError, InvalidRaffle,InvalidFee,
    InvalidAuth, InitializerNotSigner, InvalidConfig, NotSignerAuth, InvalidInitializer };

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: RaffleProgramInstruction = RaffleProgramInstruction::unpack(instruction_data)?;

        match instruction {
            RaffleProgramInstruction::InitRaffle {init_raffle}=> {
                Self::init_raffle(accounts, program_id, init_raffle)
            },
            RaffleProgramInstruction::PublishWinner => {
                Self::publish_winner(accounts, program_id)
            },
            RaffleProgramInstruction::InitCounter => {
                Self::init_raffle_counter(accounts, program_id)
            },
            RaffleProgramInstruction::InitFee {data}=> {
                Self::init_fee_account(accounts, program_id, data)
            },
            RaffleProgramInstruction::InitConfig => {
                Self::init_config(accounts, program_id)
            },
            RaffleProgramInstruction::SetConfig => {
                Self::set_config(accounts, program_id)
            },
            RaffleProgramInstruction::UpdateFee {data}=> {
                Self::update_fee(accounts, program_id, data)
            },
            RaffleProgramInstruction::CollectFee => {
                Self::collect_fee(accounts, program_id)
            },

        }
    }

    fn init_raffle(
        accounts: &[AccountInfo],program_id: &Pubkey, init_raffle:Raffle
    ) -> ProgramResult{


       let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

       let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
       let raffle_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
       let raffle_counter: &AccountInfo<'_> = next_account_info(accounts_iter)?;
       let entropy_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
       let rng_fee_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
       let fee_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
       let rng_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
       let system_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;


       if !initializer.is_signer{return Err(InvalidAuth.into());}

       let fee: Fee = Fee::try_from_slice(&fee_account.data.borrow())?;


       let mut counter: RaffleCounter = RaffleCounter::try_from_slice(&raffle_counter.data.borrow())?;

       if raffle_counter.owner != program_id{return Err(InvalidCounter.into());}
       if fee_account.owner != program_id{return Err(InvalidCounter.into());}
       if !initializer.is_signer{return Err(InitializerNotSigner.into())}
       if fee.fee == 0{return Err(InvalidFee.into());}
       if counter.initialized != 1 {return Err(InvalidCounter.into());}

       counter.number_of_raffles = counter.number_of_raffles.checked_add(1).ok_or(ArithmeticError)?;

       let (raffle_account_address, bump) = 
       Pubkey::find_program_address(&[b"raffle", &counter.number_of_raffles.to_le_bytes()], program_id);

        let rent: Rent = Rent::default();
        let rent_amount: u64 = rent.minimum_balance(153);

        invoke_signed(
            &system_instruction::create_account(
                initializer.key,
                &raffle_account_address,
                rent_amount,
                153,
                program_id,
            ),
            &[initializer.clone(), raffle_account.clone()],
            &[&[b"raffle", &counter.number_of_raffles.to_le_bytes(), &[bump]]],
        )?;

        let create_ix: solana_program::instruction::Instruction = system_instruction::transfer(
            initializer.key,
            fee_account.key,
            fee.fee
        );

        invoke(
        &create_ix,
        &[initializer.clone(), fee_account.clone(), system_program.clone()],
        )?;

        //Creating account metas for CPI to RNG_PROGRAM
        let initializer_meta: AccountMeta = AccountMeta{ pubkey: *initializer.key, is_signer: true, is_writable: true,};
        let entropy_account_meta: AccountMeta = AccountMeta{ pubkey: *entropy_account.key, is_signer: false, is_writable: true,};
        let fee_account_meta: AccountMeta = AccountMeta{ pubkey: *rng_fee_account.key, is_signer: false, is_writable: true,};
        let system_program_meta: AccountMeta = AccountMeta{ pubkey: *system_program.key, is_signer: false, is_writable: false,};


        //Creating instruction to cpi RNG PROGRAM
        let ix:Instruction = Instruction { program_id: *rng_program.key,
           accounts: [
            initializer_meta,
            entropy_account_meta,
            fee_account_meta,
            system_program_meta,
           ].to_vec(), data: [100].to_vec() };

        //CPI to RNG_PROGRAM
        invoke(&ix, 
          &[
            initializer.clone(),
            entropy_account.clone(),
            rng_fee_account.clone(),
            system_program.clone(),
            ])?;


        let returned_data:(Pubkey, Vec<u8>)= get_return_data().unwrap();


        //Random number is returned from the RNG_PROGRAM
        let random_number:RandomNumber;
        if &returned_data.0 == rng_program.key{
          random_number = RandomNumber::try_from_slice(&returned_data.1)?;
          msg!("{}",random_number.random_number);
        }else{
            panic!();
        }

        let winner_no: u64 = random_number.random_number%init_raffle.number_of_participants;


        let raffle: Raffle = Raffle{
            is_published:0,
            number_of_participants: init_raffle.number_of_participants,
            initializer:initializer.key.to_bytes(),
            winner_no,
            winner_wallet: [0;32],
            raffle_name: init_raffle.raffle_name,
            raffle_no: counter.number_of_raffles,
            participants_hash: init_raffle.participants_hash,
        };

        raffle.serialize(&mut &mut raffle_account.data.borrow_mut()[..])?;
        counter.serialize(&mut &mut raffle_counter.data.borrow_mut()[..])?;

        Ok(())
    }

    fn publish_winner(
        accounts: &[AccountInfo],program_id: &Pubkey
    ) -> ProgramResult{

        let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

        let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
        let raffle_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
        let winner: &AccountInfo<'_> = next_account_info(accounts_iter)?;


        let mut raffle: Raffle = Raffle::try_from_slice(&raffle_account.data.borrow())?;

        if raffle_account.owner != program_id {return Err(InvalidRaffle.into());}
        if raffle.initializer != initializer.key.to_bytes(){return Err(InvalidInitializer.into());}
        if !initializer.is_signer{return Err(InitializerNotSigner.into())}

        if raffle_account.owner != program_id {return Err(InvalidRaffle.into());}


        raffle.winner_wallet = winner.key.to_bytes();
        raffle.is_published = 1;


        raffle.serialize(&mut &mut raffle_account.data.borrow_mut()[..])?;
        
        Ok(())
    }

    fn init_raffle_counter(
        accounts: &[AccountInfo],program_id: &Pubkey
    ) -> ProgramResult{


        let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

        let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
        let counter_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;


        let rent: Rent = Rent::default();
        let rent_amount: u64 = rent.minimum_balance(9);


       let (counter_account_address, bump) = 
       Pubkey::find_program_address(&[b"counter" ], program_id);

        invoke_signed(
            &system_instruction::create_account(
                initializer.key,
                &counter_account_address,
                rent_amount,
                9,
                program_id,
            ),
            &[initializer.clone(), counter_account.clone()],
            &[&[b"counter", &[bump]]],
        )?;

        let counter = RaffleCounter{ 
            initialized:1,
            number_of_raffles: 0
         };

         counter.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;
        

        Ok(())
    }

    fn init_config(
        accounts: &[AccountInfo], program_id: &Pubkey
    ) -> ProgramResult {
    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let authority_1: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let authority_2: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let authority_3: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let authority_4: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    let (config_address, bump) = Pubkey::find_program_address( &[b"config"], program_id);

    let rent: Rent = Rent::default();
    let rent_amount: u64 = rent.minimum_balance(128);

    if config_account.owner != program_id {
        invoke_signed(
            &system_instruction::create_account(
                authority_1.key,
                &config_address,
                rent_amount,
                128,
                program_id,
            ),
            &[authority_1.clone(), config_account.clone()],
            &[&[b"config", &[bump]]],
        )?;
    }


    if !authority_1.is_signer {
        return Err(NotSignerAuth.into());
    }

    let config_data: Config = Config {
        authority_1: authority_1.key.to_bytes(),
        authority_2: authority_2.key.to_bytes(),
        authority_3: authority_3.key.to_bytes(),
        authority_4: authority_4.key.to_bytes(),
    };

    config_data.serialize(&mut &mut config_account.data.borrow_mut()[..])?;

    Ok(())
}

    fn set_config(
        accounts: &[AccountInfo], program_id: &Pubkey
    ) -> ProgramResult {
    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let authority: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let authority_1: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let authority_2: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let authority_3: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let authority_4: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    

    if config_account.owner != program_id {
        return Err(InvalidConfig.into());
    }
    

    let config: Config = Config::try_from_slice(&config_account.data.borrow())?;


    Self::check_authority(authority.key, config)?;
    

    if !authority.is_signer {
        return Err(NotSignerAuth.into());
    }
    

    let config_data: Config = Config {
        authority_1: authority_1.key.to_bytes(),
        authority_2: authority_2.key.to_bytes(),
        authority_3: authority_3.key.to_bytes(),
        authority_4: authority_4.key.to_bytes(),
    };
    

    config_data.serialize(&mut &mut config_account.data.borrow_mut()[..])?;

    Ok(())
}

    fn init_fee_account(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data: InitPda,
    ) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let authority: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let fee_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if config_account.owner != program_id {
        return Err(InvalidConfig.into());
    }

    let config: Config = Config::try_from_slice(&config_account.data.borrow())?;


    Self::check_authority(authority.key, config)?;

    if !authority.is_signer {
        return Err(NotSignerAuth.into());
    }

    let (fee_account_pubkey, bump) = Pubkey::find_program_address(&[b"fee"], program_id);

    let create_ix: solana_program::instruction::Instruction =
        system_instruction::create_account(
            authority.key,
            &fee_account_pubkey,
            data.lamports,
            8,
            program_id,
        );

    invoke_signed(
        &create_ix,
        &[authority.clone(), fee_account.clone()],
        &[&[b"fee", &[bump]]],
    )?;

    let fee_collector: Fee = Fee { fee: 250000  };

    fee_collector.serialize(&mut &mut fee_account.data.borrow_mut()[..])?;

    Ok(())
}

    fn update_fee(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data: Fee,
    ) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let authority: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let fee_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    

    if config_account.owner != program_id {
        return Err(InvalidConfig.into());
    }
    

    let config: Config = Config::try_from_slice(&config_account.data.borrow())?;

    

    Self::check_authority(authority.key, config)?;

    if !authority.is_signer {
        return Err(NotSignerAuth.into());
    }

    


    data.serialize(&mut &mut fee_account.data.borrow_mut()[..])?;

    Ok(())
}

    fn collect_fee(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    ) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let authority: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let fee_collector: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if config_account.owner != program_id {
        return Err(InvalidConfig.into());
    }


    let config: Config = Config::try_from_slice(&config_account.data.borrow())?;

    Self::check_authority(authority.key, config)?;

    if !authority.is_signer {
        return Err(NotSignerAuth.into());
    }

    let value: u64 = **fee_collector.lamports.borrow();

    let collected_fee: u64 = value.checked_sub(2500000).ok_or(ArithmeticError)?;

    **fee_collector.try_borrow_mut_lamports()? -= collected_fee;
    **authority.try_borrow_mut_lamports()? += collected_fee;
    
    Ok(())
   }


   
    fn check_authority(
        authority: &Pubkey, config: Config
    ) -> ProgramResult {
        let authority_address_1: Pubkey = Pubkey::new_from_array(config.authority_1);
        let authority_address_2: Pubkey = Pubkey::new_from_array(config.authority_2);
        let authority_address_3: Pubkey = Pubkey::new_from_array(config.authority_3);
        let authority_address_4: Pubkey = Pubkey::new_from_array(config.authority_4);
    
        let valid_authorities: [Pubkey; 4] = [
            authority_address_1,
            authority_address_2,
            authority_address_3,
            authority_address_4,
        ];
    
        if !valid_authorities.contains(authority) {
            return Err(InvalidAuth.into());
        }
    
        Ok(())
    }
    

}


//ceklis kayit acik - 1
//cekilis yapildi  - 2
//kazanan yayinlandi - 3

