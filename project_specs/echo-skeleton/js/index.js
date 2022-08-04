import { Connection, sendAndConfirmTransaction, Keypair, Transaction, SystemProgram, PublicKey, TransactionInstruction } from "@solana/web3.js";
import { readFileSync } from "fs";

import BN from "bn.js";
const pathToMyKeypair = process.env.HOME + "/.config/solana/phantom.json";
const keypairFile = readFileSync(pathToMyKeypair);
const secretKey = Buffer.from(JSON.parse(keypairFile.toString()));
const feePayer = Keypair.fromSecretKey(secretKey);

const main = async () => {
  var args = process.argv.slice(2);
  const programId = new PublicKey(args[0]);
  const echo = args[1];

  const connection = new Connection("https://api.devnet.solana.com/");
  const echoBuffer = new Keypair();

  console.log("Requesting Airdrop of 1 SOL...");
  await connection.requestAirdrop(feePayer.publicKey, 2e9);
  console.log("Airdrop received");

  let createIx = SystemProgram.createAccount({
    fromPubkey: feePayer.publicKey,
    newAccountPubkey: echoBuffer.publicKey,
    lamports: await connection.getMinimumBalanceForRentExemption(echo.length),
    space: echo.length,
    programId: programId,
  });

  const idx = Buffer.from(new Uint8Array([0]));
  const messageLen = Buffer.from(
    new Uint8Array(new BN(echo.length).toArray("le", 4))
  );
  const message = Buffer.from(echo, "ascii");

  let echoIx = new TransactionInstruction({
    keys: [
      {
        pubkey: echoBuffer.publicKey,
        isSigner: false,
        isWritable: true,
      },
    ],
    programId: programId,
    data: Buffer.concat([idx, messageLen, message]),
  });

  let tx = new Transaction();
  tx.add(createIx).add(echoIx);

  let txid = await sendAndConfirmTransaction(
    connection,
    tx,
    [feePayer, echoBuffer],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    }
  );
  console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet`);

  data = (await connection.getAccountInfo(echoBuffer.publicKey)).data;
  console.log("Echo Buffer Text:", data.toString());
};

main()
  .then(() => {
    console.log("Success");
  })
  .catch((e) => {
    console.error(e);
  });
