import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VestingContract } from "../target/types/vesting_contract";




import { PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("vesting_contract", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.SimpleSolContract as Program<VestingContract>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const wallet = provider.wallet;

  let contractAccount: PublicKey;

  it("Initializes the contract", async () => {
    
    const contractKeypair = anchor.web3.Keypair.generate();
    contractAccount = contractKeypair.publicKey;

    const tx = await program.methods
      .initialize()
      .accounts({
        contractAccount: contractAccount,
        user: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([contractKeypair])
      .rpc();

    console.log("Contract initialized successfully!");
    console.log("Contract Account:", contractAccount.toBase58());
  });

  it("Receives SOL and locks for 10 seconds", async () => {
    const amount = 1 * anchor.web3.LAMPORTS_PER_SOL; 
    const timestampBefore = Math.floor(Date.now() / 1000);

    const tx = await program.methods
      .receiveSol(new anchor.BN(amount))
      .accounts({
        contractAccount: contractAccount,
        sender: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const contractData = await program.account.contractAccount.fetch(contractAccount);
    console.log("SOL Received Successfully!");
    console.log("Amount Received:", amount / anchor.web3.LAMPORTS_PER_SOL, "SOL");
    console.log("Timestamp:", timestampBefore, "seconds");

    assert.strictEqual(contractData.totalReceived.toNumber(), amount);
    assert.isTrue(contractData.lastReceivedTimestamp > 0);
    console.log("Contract State Updated: Total Received:", contractData.totalReceived.toNumber() / anchor.web3.LAMPORTS_PER_SOL, "SOL");
  });

  it("Sends back SOL after 10 seconds", async () => {
    const amount = 1 * anchor.web3.LAMPORTS_PER_SOL;

    console.log("Waiting for 10 seconds...");
    await new Promise((resolve) => setTimeout(resolve, 10000)); 
    const timestampBefore = Math.floor(Date.now() / 1000);

    const tx = await program.methods
      .sendBackSol(new anchor.BN(amount))
      .accounts({
        contractAccount: contractAccount,
        recipient: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const timestampAfter = Math.floor(Date.now() / 1000);

    console.log("SOL Sent Back Successfully!");
    console.log("Amount Sent Back:", amount / anchor.web3.LAMPORTS_PER_SOL, "SOL");
    console.log("Timestamp Before:", timestampBefore, "seconds");
    console.log("Timestamp After:", timestampAfter, "seconds");
    assert.isTrue(timestampAfter - timestampBefore >= 10, "Lock period not enforced.");
  });
});
