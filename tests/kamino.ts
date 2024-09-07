import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {
  KaminoMarket,
  KaminoAction,
  VanillaObligation,
  PROGRAM_ID,
  DEFAULT_RECENT_SLOT_DURATION_MS,
} from "@kamino-finance/klend-sdk";
import fs from "fs";
import BN from "bn.js";
//import { KaminoRepay } from "../target/types/kamino_repay";

//const provider = anchor.AnchorProvider.env();
//anchor.setProvider(provider);

const MAINNET_LENDING_MARKET = new PublicKey(
  "7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF"
);

const FARM = new PublicKey("FarmsPZpWu9i7Kky8tPN37rs2TpmMrAZrC7S7vJa91Hr");
const KAMINO_PROGRAM = new PublicKey(
  "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD"
);

const connection = new Connection(
  "https://mainnet.helius-rpc.com/?api-key=91acf6dc-f1f0-4db8-9763-aff8b775fa6a"
);
//@ts-ignore
//const program = anchor.workspace.KaminoRepay as anchor.Program<KaminoRepay>;
describe("Exec Kamino", () => {
  let payer = Keypair.generate();
  const repayAmt = new BN(1 * 10 ** 9);

  it("repay", async () => {
    try {
      const kaminoMarket = await KaminoMarket.load(
        connection,
        MAINNET_LENDING_MARKET,
        DEFAULT_RECENT_SLOT_DURATION_MS,
        PROGRAM_ID,
        true
      );

      const kaminoAction = await KaminoAction.buildRepayTxns(
        kaminoMarket,
        repayAmt,
        new PublicKey("So11111111111111111111111111111111111111112"),
        new PublicKey("HxW4EmTr2WhvrKuxXjB5t5hEpftNebxw6mXsVxcYDMHT"),
        new VanillaObligation(PROGRAM_ID),
        1_000_000,
      );

      // console.log(kaminoAction.preTxnIxs); // directly send this to blockchain

      const allInstructions = [
        ...kaminoAction.setupIxs,
        ...kaminoAction.lendingIxs,
        ...kaminoAction.cleanupIxs,
      ];
      console.log(__dirname)
      fs.writeFile(__dirname+"/e.json", JSON.stringify(allInstructions, null, 4), () => {
        console.log("done")
      });

      // //Handling only kamino program ixs for now
      // const kaminoIxs = allInstructions.filter((ix) =>
      //   ix.programId.equals(KAMINO_PROGRAM)
      // );

      // //TODO: Filter unique accounts
      // const allAccountMetas = kaminoIxs.flatMap((ix) => ix.keys);

      // const ixDatas = kaminoIxs.map((ix) => ix.data);

      // // Send transaction using anchor program
      // const txn = await program.methods
      //   .executeKaminoOperations({ ixDatas })
      //   .accounts({
      //     kaminoFarm: FARM,
      //     kaminoProgram: KAMINO_PROGRAM,
      //   })
      //   .remainingAccounts(allAccountMetas)
      //   .signers([payer])
      //   .rpc();

      // console.log("Transaction signature:", txn);
    } catch (error) {
      console.error("Error:", error);
    }
  });
});