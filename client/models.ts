

  export  class Raffle{

    participants_hash:number[] = Array.from({length: 32}, () => 1);
    initializer:number[] = Array.from({length: 32}, () => 1);
    raffle_no:bigint = BigInt(0);
    number_of_participants:bigint = BigInt(0);
    winner_no:bigint = BigInt(0);
    winner_wallet:number[] = Array.from({length: 32}, () => 1);
    raffle_name:number[] = Array.from({length: 32}, () => 1);

    constructor(fields: {

        participants_hash:number[];
        initializer:number[];
        raffle_no:bigint;
        number_of_participants:bigint;
        winner_no:bigint;
        winner_wallet:number[];
        raffle_name:number[];
    
  
     } | undefined = undefined)
      {if (fields) {
        this.participants_hash = fields.participants_hash;
        this.initializer = fields.initializer;
        this.raffle_no = fields.raffle_no;
        this.number_of_participants = fields.number_of_participants;
        this.winner_no = fields.winner_no;
        this.winner_wallet = fields.winner_wallet;
        this.raffle_name = fields.raffle_name;

      }
    }
  }
  
  export  const RaffleSchema =new Map([
    [
      Raffle,
      {
        kind: "struct",
        fields: [
          ["participants_hash",["u8",32]],
          ["initializer",["u8",32]],
          ["raffle_no","u64"],
          ["number_of_participants","u64"],
          ["winner_no","u64"],
          ["winner_wallet",["u8",32]],
          ["raffle_name",["u8",32]],
    ],
  },
  ],
  ])


  export  class Counter{

    initialized:number = 0;
    number_of_raffles:bigint = BigInt(0);


    constructor(fields: {

    initialized:number;
    number_of_raffles:bigint;
    
  
     } | undefined = undefined)
      {if (fields) {

        this.initialized = fields.initialized;
        this.number_of_raffles = fields.number_of_raffles;


      }
    }
  }
  export  const CounterSchema =new Map([
    [
      Counter,
      {
        kind: "struct",
        fields: [

          ["initialized","u8"],
          ["number_of_raffles","u64"],

    ],
  },
  ],
  ])

  export  class RaffleName{


    raffle_name:number[] = Array.from({length: 32}, () => 1);

    constructor(fields: {


        raffle_name:number[];
    
  
     } | undefined = undefined)
      {if (fields) {

        this.raffle_name = fields.raffle_name;

      }
    }
  }
  export  const RaffleNameSchema =new Map([
    [
      RaffleName,
      {
        kind: "struct",
        fields: [

          ["raffle_name",["u8",32]],
    ],
  },
  ],
  ])

  export   class InitPda{
    bump:number = 0;
    lamports:number = 0
    no:number = 0;
    constructor(fields: {
      bump:number;
      lamports:number;
      no:number;
     } | undefined = undefined)
      {if (fields) {
        this.lamports = fields.lamports;
        this.bump = fields.bump;
        this.no = fields.no;
      }
    }
  }
  export   const InitPdaSchema =new Map([
    [
      InitPda,
      {
        kind: "struct",
        fields: [
          ["bump","u8"],
          ["lamports","u64"],
          ["no","u8"],
    ],
  },
  ],
  ])
  
  export  class Config{
  
    authority1:number[] = Array.from({ length: 32 }, () => 1);
    authority2:number[] = Array.from({ length: 32 }, () => 1);
    authority3:number[] = Array.from({ length: 32 }, () => 1);
    authority4:number[] = Array.from({ length: 32 }, () => 1);
  
    constructor(fields: {
      authority1:number[];
      authority2:number[];
      authority3:number[];
      authority4:number[];
  
     } | undefined = undefined)
      {if (fields) {
        this.authority1 = fields.authority1;
        this.authority2 = fields.authority2;
        this.authority3 = fields.authority3;
        this.authority4 = fields.authority4;
  
      }
    }
  }
  export  const ConfigSchema =new Map([
    [
      Config,
      {
        kind: "struct",
        fields: [
          ["authority1",["u8",32]],
          ["authority2",["u8",32]],
          ["authority3",["u8",32]],
          ["authority4",["u8",32]],
    ],
  },
  ],
  ])
    
  export  class Fee{
  
    fee:bigint = BigInt(0);
    constructor(fields: {
  
      fee:bigint;
     } | undefined = undefined)
      {if (fields) {
  
        this.fee = fields.fee;
      }
    }
  }
  export  const FeeSchema =new Map([
    [
        Fee,
      {
        kind: "struct",
        fields: [
          ["fee","u64"],
    ],
  },
  ],
  ])