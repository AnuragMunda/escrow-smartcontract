import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EscrowContract } from "../target/types/escrow_contract";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import {createAssociatedTokenAccount, createMint, getAccount, getAssociatedTokenAddress, mintTo} from "@solana/spl-token";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.escrowContract as Program<EscrowContract>;
  let provider: anchor.Provider;

  let initialiser: anchor.Wallet;
  let taker = anchor.web3.Keypair.generate();

  let mintA: PublicKey;
  let mintB: PublicKey;

  let initialiserTokenAccountA: PublicKey;
  let initialiserTokenAccountB: PublicKey;
  let takerTokenAccountA: PublicKey;
  let takerTokenAccountB: PublicKey;

  let vaultAccount: PublicKey;
  let escrowAccount: anchor.web3.Keypair;
  let escrowPda: PublicKey;
  let escrowBump: number;

  const amount_a = 1_000_000;
  const amount_b = 500_000;

  before(async () => {
    provider = anchor.getProvider();
    initialiser = provider.wallet as anchor.Wallet;
    const airdropSignature = await provider.connection.requestAirdrop(
      initialiser.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 10
    );
    await provider.connection.confirmTransaction(airdropSignature);

    // Create two SPL tokens
    mintA = await createMint(provider.connection, initialiser.payer, initialiser.publicKey, null, 6);
    mintB = await createMint(provider.connection, initialiser.payer, initialiser.publicKey, null, 6);

    // Create Token accounts
    initialiserTokenAccountA = await createAssociatedTokenAccount(
      provider.connection,
      initialiser.payer,
      mintA,
      initialiser.publicKey
    );

    initialiserTokenAccountB = await createAssociatedTokenAccount(
      provider.connection,
      initialiser.payer,
      mintB,
      initialiser.publicKey
    );

    takerTokenAccountA = await createAssociatedTokenAccount(
      provider.connection,
      initialiser.payer,
      mintA,
      taker.publicKey
    );

    takerTokenAccountB = await createAssociatedTokenAccount(
      provider.connection,
      initialiser.payer,
      mintB,
      taker.publicKey
    )

    // Mint tokens
    await mintTo(
      provider.connection,
      initialiser.payer,
      mintA,
      initialiserTokenAccountA,
      initialiser.payer,
      amount_a
    );

    await mintTo(
      provider.connection,
      initialiser.payer,
      mintB,
      takerTokenAccountB,
      initialiser.payer,
      amount_b
    );

    // Create escrow account + PDA
    escrowAccount = anchor.web3.Keypair.generate();
    [escrowPda, escrowBump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), escrowAccount.publicKey.toBuffer()],
      program.programId
    );

    // Vault ATA for PDA
    vaultAccount = await getAssociatedTokenAddress(mintA, escrowPda, true);
    
    await createAssociatedTokenAccount(
      provider.connection,
      initialiser.payer,
      mintA,
      escrowPda,
    );
  })

  it("should initialise escrow", async () => {
    await program.methods.initialiseEscrow(new anchor.BN(amount_a), new anchor.BN(amount_b))
    .accounts({
      initialiser: initialiser.publicKey,
      initialiserTokenAccount: initialiserTokenAccountA,
      initialiserReceiveTokenAccount: initialiserTokenAccountB,
      vaultAccount,
      escrow: escrowAccount.publicKey,
      mintA: mintA
      // escrow_pda: escrowPda,
      // tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      // systemProgram: SystemProgram.programId,
      // rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .signers([escrowAccount])
    .rpc();

    const escrowState = await program.account.escrow.fetch(escrowAccount.publicKey);
    console.log("Escrow state after initialise", escrowState);

    const vaultInfo = await getAccount(provider.connection, vaultAccount);
    console.log("Vault balance:", Number(vaultInfo.amount));
     if (Number(vaultInfo.amount) !== amount_a) {
      throw new Error("Vault did not receive correct amount of token A");
    }
  })
});
