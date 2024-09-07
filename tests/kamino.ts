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
import BN from "bn.js";
import { KaminoRepay } from "../target/types/kamino_repay";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

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
const program = anchor.workspace.KaminoRepay as anchor.Program<KaminoRepay>;

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

      const allInstructions = [
        ...kaminoAction.setupIxs,
        ...kaminoAction.lendingIxs,
        ...kaminoAction.cleanupIxs,
      ];

      const kaminoIxs = allInstructions.filter((ix) =>
        ix.programId.equals(KAMINO_PROGRAM)
      );

      // Filter unique accounts
      const uniqueAccounts = new Map();
      kaminoIxs.forEach((ix) => {
        ix.keys.forEach((accountMeta) => {
          const key = accountMeta.pubkey.toBase58();
          if (!uniqueAccounts.has(key)) {
            uniqueAccounts.set(key, accountMeta);
          } else {
            // Merge isSigner and isWritable flags
            const existing = uniqueAccounts.get(key);
            existing.isSigner = existing.isSigner || accountMeta.isSigner;
            existing.isWritable = existing.isWritable || accountMeta.isWritable;
          }
        });
      });
      const allAccountMetas = Array.from(uniqueAccounts.values());

      const ixDatas = kaminoIxs.map((ix) => ix.data);

      // Send transaction using anchor program
      // const txn = await program.methods
      //   .executeKaminoRepay({ ixDatas })
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