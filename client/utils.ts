import { deserialize, serialize } from "borsh";
import { CounterSchema, Counter, RaffleSchema, Raffle } from "./models";
import { AccountInfo, PublicKey } from "@solana/web3.js";
import { connection } from "./connection";


export function numberToLEBytes8(num: bigint): Uint8Array {
    if (num < 0n || num > 0xFFFFFFFFFFFFFFFFn) {
        throw new RangeError("Number out of range for 8-byte conversion.");
    }

    const buffer = new ArrayBuffer(8); // 8 bytes for 64 bits
    const dataView = new DataView(buffer);

    for (let i = 0; i < 8; i++) {
        dataView.setUint8(i, Number(num & 0xFFn)); // Extract least significant byte
        num >>= 8n; // Shift right by 8 bits
    }

    return new Uint8Array(buffer);
}

export function stringToNumberArray32Bytes(input: string): number[] {
  // Convert string to UTF-8 encoded bytes
  const encoder = new TextEncoder();
  let bytes = Array.from(encoder.encode(input));

  // Ensure the array is exactly 32 bytes
  if (bytes.length > 32) {
      bytes = bytes.slice(0, 32); // Truncate to 32 bytes
  } else if (bytes.length < 32) {
      while (bytes.length < 32) {
          bytes.push(0); // Pad with zeros
      }
  }

  return bytes;
}

export function deserialize_raffle_account_data(account_info:AccountInfo<Buffer>){

    const raffle = deserialize(RaffleSchema,Raffle,account_info.data);

    console.log("raffle no = " + raffle.raffle_no.toString())
    console.log("number of participants = " + raffle.number_of_participants.toString())
    console.log("winner no = " + raffle.winner_no.toString())
    console.log("initializer = " + new PublicKey(raffle.initializer).toBase58())
    console.log("winner  = " + new PublicKey(raffle.winner_wallet).toBase58())

    return raffle;


}


export function deserialize_counter_account_data(account_info:AccountInfo<Buffer>){

    const counter = deserialize(CounterSchema,Counter,account_info.data);

    return counter;

}


