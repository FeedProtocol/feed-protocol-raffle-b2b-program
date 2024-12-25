import {
    Keypair,
    PublicKey,
    TransactionMessage,
    VersionedTransaction,
    SystemProgram,
    TransactionInstruction,
    LAMPORTS_PER_SOL,

  } from "@solana/web3.js";

  import { deserialize, serialize } from "borsh";
  import {  Counter, CounterSchema, Fee, FeeSchema, InitPda, InitPdaSchema, Raffle, RaffleName, RaffleSchema } from "./models";
  import {connection} from './connection';
  import { raffle_program, entropy_account, rng_fee_account, rng_program, ugur, } from "./accounts"
  import { deserialize_counter_account_data, deserialize_raffle_account_data, numberToLEBytes8, stringToNumberArray32Bytes } from "./utils";
  import { get_all_raffles, get_raffle_by_raffle_no } from "./get_info";


  const init_raffle = async (raffle_name:string, number_of_participants:bigint, participant_hash:number[],authority:Keypair) => {
  
  
       const counter_account = PublicKey.findProgramAddressSync([Buffer.from("counter")],raffle_program)[0];

       const counter_info = await connection.getAccountInfo(counter_account);

       const counter = deserialize(CounterSchema,Counter,counter_info?.data!);

       counter.number_of_raffles = BigInt(Number(counter.number_of_raffles) + 1);

       const le_bytes = numberToLEBytes8(counter.number_of_raffles);

       const raffle_account = PublicKey.findProgramAddressSync([Buffer.from("raffle"),le_bytes],raffle_program)[0];

       const raffle_name_array = stringToNumberArray32Bytes(raffle_name);

       const raffle = new Raffle();

       raffle.number_of_participants = number_of_participants;
       raffle.raffle_name = raffle_name_array;
       raffle.participants_hash = participant_hash;

       const raffle_serialized = serialize(RaffleSchema,raffle);

       const concated = Uint8Array.of(0, ...raffle_serialized);

       const fee_account = PublicKey.findProgramAddressSync([Buffer.from("fee")],raffle_program)[0];

       const ix = new TransactionInstruction({
         programId: raffle_program,
         keys: [
           { isSigner: true, isWritable: true, pubkey: authority.publicKey },
           { isSigner: false, isWritable: true, pubkey: raffle_account },
           { isSigner: false, isWritable: true, pubkey: counter_account },
           { isSigner: false, isWritable: true, pubkey: entropy_account },
           { isSigner: false, isWritable: true, pubkey: rng_fee_account },
           { isSigner: false, isWritable: true, pubkey: fee_account },
           { isSigner: false, isWritable: false, pubkey: rng_program },
           { isSigner: false, isWritable: false, pubkey: SystemProgram.programId },
      ],
         data: Buffer.from(concated)
       });

       const message = new TransactionMessage({
         instructions: [ix],
         payerKey: authority.publicKey,
         recentBlockhash: (await connection.getLatestBlockhash()).blockhash
       }).compileToV0Message();

       const tx = new VersionedTransaction(message);
       tx.sign([authority]);

       const sig = connection.sendTransaction(tx);

       console.log("raffle = " + raffle_account.toBase58())
       console.log("counter = " + counter_account.toBase58())

       console.log(sig)
  }

  const publish_winner = async (raffle_no:bigint,authority:Keypair,winner:PublicKey,winner_as_bytes:number[]) => {


    const le_bytes = numberToLEBytes8(raffle_no)
 
    const raffle_account = PublicKey.findProgramAddressSync([Buffer.from("raffle"),le_bytes],raffle_program)[0];

 
    const config_account = PublicKey.findProgramAddressSync([Buffer.from("config")],raffle_program)[0];




     const ix = new TransactionInstruction({
       programId: raffle_program,
       keys: [
         { isSigner: true, isWritable: true, pubkey: authority.publicKey },
         { isSigner: false, isWritable: true, pubkey: raffle_account },
         { isSigner: false, isWritable: false, pubkey: config_account },
       ],
       data: Buffer.from([1,...winner_as_bytes])
     });
   
     const message = new TransactionMessage({
       instructions: [ix],
       payerKey: authority.publicKey,
       recentBlockhash: (await connection.getLatestBlockhash()).blockhash
     }).compileToV0Message();
   
     const tx = new VersionedTransaction(message);
     tx.sign([authority]);
 
    const s  = await connection.sendTransaction(tx);

    console.log(s)
     
  }

  const init_counter = async (authority:Keypair) => {
  
  

    const counter_account = PublicKey.findProgramAddressSync([Buffer.from("counter")],raffle_program)[0]
   
     const ix = new TransactionInstruction({
       programId: raffle_program,
       keys: [
         { isSigner: true, isWritable: true, pubkey: authority.publicKey },
         { isSigner: false, isWritable: true, pubkey: counter_account },
         { isSigner: false, isWritable: false, pubkey: SystemProgram.programId },
 
       ],
       data: Buffer.from([2])
     });
   
     const message = new TransactionMessage({
       instructions: [ix],
       payerKey: authority.publicKey,
       recentBlockhash: (await connection.getLatestBlockhash()).blockhash
     }).compileToV0Message();
   
     const tx = new VersionedTransaction(message);
     tx.sign([authority]);
 
     const sig = await connection.sendTransaction(tx);

     console.log(sig)

     console.log(counter_account.toBase58())
  }

  const close_pda = async (raffle_no:bigint,participant_no:bigint,authority:Keypair) => {
  
  
   
    const raffle_no_le_bytes = numberToLEBytes8(raffle_no)
 
    const raffle_account = PublicKey.findProgramAddressSync([Buffer.from("raffle"),raffle_no_le_bytes],raffle_program)[0];
 
 
    const participant_no_le_bytes = numberToLEBytes8(participant_no);

 
    const participant_pda = PublicKey.findProgramAddressSync(
     [Buffer.from("part"), participant_no_le_bytes,
     Buffer.from("raf"),raffle_no_le_bytes],raffle_program)[0];
   
     const ix = new TransactionInstruction({
       programId: raffle_program,
       keys: [
         { isSigner: true, isWritable: true, pubkey: authority.publicKey },
         { isSigner: false, isWritable: true, pubkey: raffle_account },
         { isSigner: false, isWritable: true, pubkey: participant_pda },
 
       ],
       data: Buffer.from([5])
     });
   
     const message = new TransactionMessage({
       instructions: [ix],
       payerKey: authority.publicKey,
       recentBlockhash: (await connection.getLatestBlockhash()).blockhash
     }).compileToV0Message();
   
     const tx = new VersionedTransaction(message);
     tx.sign([authority]);
 
     connection.sendTransaction(tx);
  }

  const set_config = async (authority:Keypair) => {
  
    const config_account = PublicKey.findProgramAddressSync([Buffer.from("config")],raffle_program)[0];

  
    const ix = new TransactionInstruction({
      programId: raffle_program,
      keys: [
        { isSigner: true, isWritable: true, pubkey: authority.publicKey },
        { isSigner: false, isWritable: false, pubkey: ugur },//1
        { isSigner: false, isWritable: false, pubkey: authority.publicKey },//2
        { isSigner: false, isWritable: false, pubkey: authority.publicKey },//3
        { isSigner: false, isWritable: false, pubkey: authority.publicKey },//4
        { isSigner: false, isWritable: true, pubkey: config_account },
      ],
      data: Buffer.from([5])
    });
  

    const message = new TransactionMessage({
      instructions: [ix],
      payerKey: authority.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    const tx = new VersionedTransaction(message);
    tx.sign([authority]);
  
    const sig = await connection.sendTransaction(tx);

    console.log(sig)
  
  }

   const collect_fee = async (authority:Keypair) => {
  
    const fee_account = PublicKey.findProgramAddressSync([Buffer.from("fee")], raffle_program)[0];
  
    const config_account = PublicKey.findProgramAddressSync([Buffer.from("config")],raffle_program)[0];

    const ix = new TransactionInstruction({
      programId: raffle_program,
      keys: [
        { isSigner: true, isWritable: true, pubkey: authority.publicKey },
        { isSigner: false, isWritable: true, pubkey: fee_account },
        { isSigner: false, isWritable: false, pubkey: config_account },
      ],
      data: Buffer.from([7])
    });
  
    const message = new TransactionMessage({
      instructions: [ix],
      payerKey: authority.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    const tx = new VersionedTransaction(message);
    tx.sign([authority]);
  
    const sig = await connection.sendTransaction(tx);
  
  }

   const update_fee = async (authority:Keypair, new_fee:number) => {

    const fee = new Fee()

    fee.fee = BigInt(new_fee*LAMPORTS_PER_SOL);

    let encoded = serialize(FeeSchema, fee);
  
    const config_account = PublicKey.findProgramAddressSync([Buffer.from("config")],raffle_program)

  
    let concated = Uint8Array.of(6, ...encoded);
  
    const fee_account = PublicKey.findProgramAddressSync([Buffer.from("fee")], raffle_program);
  
    const ix = new TransactionInstruction({
      programId: raffle_program,
      keys: [
        { isSigner: true, isWritable: true, pubkey: authority.publicKey },
        { isSigner: false, isWritable: true, pubkey: fee_account[0] },
        { isSigner: false, isWritable: false, pubkey: config_account[0] },
      ],
      data: Buffer.from(concated)
    });
  
    const message = new TransactionMessage({
      instructions: [ix],
      payerKey: authority.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    const tx = new VersionedTransaction(message);
    tx.sign([authority]);
  
    const sig = await connection.sendTransaction(tx);
  
    console.log(sig)
  }

   const init_config = async (authority:Keypair) => {

    console.log(authority.publicKey.toBase58())

    const config_account = PublicKey.findProgramAddressSync([Buffer.from("config")],raffle_program)
    console.log(config_account.toString())
  
    const ix = new TransactionInstruction({
      programId: raffle_program,
      keys: [
        { isSigner: true, isWritable: true, pubkey: authority.publicKey },
        { isSigner: false, isWritable: false, pubkey: authority.publicKey  },
        { isSigner: false, isWritable: false, pubkey: authority.publicKey  },
        { isSigner: false, isWritable: false, pubkey: authority.publicKey  },
        { isSigner: false, isWritable: true, pubkey: config_account[0] },
        { isSigner: false, isWritable: false, pubkey: SystemProgram.programId },
      ],
      data: Buffer.from([4])
    });
  
    const message = new TransactionMessage({
      instructions: [ix],
      payerKey: authority.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    const tx = new VersionedTransaction(message);
    tx.sign([authority]);
  
    const sig = await connection.sendTransaction(tx);

    console.log(sig)
  
  }

   const init_fee_account = async (authority:Keypair) => {


    const fee_account = PublicKey.findProgramAddressSync([Buffer.from("fee")], raffle_program);

    console.log("fee account = " + fee_account[0]);
    console.log("fee account bump = " + fee_account[1]);

    const data = new InitPda();

    data.lamports = 0.0027*LAMPORTS_PER_SOL;

    console.log(data.bump);

    const encoded = serialize(InitPdaSchema, data);

    let concated = Uint8Array.of(3, ...encoded);

    const config_account = PublicKey.findProgramAddressSync([Buffer.from("config")],raffle_program)


    const ix = new TransactionInstruction({
      programId: raffle_program,
      keys: [
        { isSigner: true, isWritable: true, pubkey: authority.publicKey },
        { isSigner: false, isWritable: true, pubkey: fee_account[0] },
        { isSigner: false, isWritable: false, pubkey: config_account[0] },
        { isSigner: false, isWritable: false, pubkey: SystemProgram.programId },
      ],
      data: Buffer.from(concated)
    });

    const message = new TransactionMessage({
      instructions: [ix],
      payerKey: authority.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();

    const tx = new VersionedTransaction(message);
    tx.sign([authority]);

    const sig = await connection.sendTransaction(tx);

    console.log(sig)
  }

   const close_account = async (authority:Keypair) => {
  
    const counter_account = PublicKey.findProgramAddressSync([Buffer.from("counter")],raffle_program)[0]
  

    const ix = new TransactionInstruction({
      programId: raffle_program,
      keys: [
        { isSigner: true, isWritable: true, pubkey: authority.publicKey },
        { isSigner: false, isWritable: true, pubkey: counter_account },
      ],
      data: Buffer.from([8])
    })

  console.log(counter_account.toBase58())

    const message = new TransactionMessage({
      instructions: [ix],
      payerKey: authority.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    const tx = new VersionedTransaction(message);
    tx.sign([authority]);
  
    const sig = await connection.sendTransaction(tx);
  
  }
