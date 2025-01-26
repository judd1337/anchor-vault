import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";
import { BN } from "bn.js";
import { 
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID, 
  getAssociatedTokenAddressSync, 
  getMinimumBalanceForRentExemptMint, 
} from "@solana/spl-token";


describe("vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  let connection = anchor.getProvider().connection;

  const program = anchor.workspace.Vault as Program<Vault>;
  const signer = anchor.web3.Keypair.generate(); // Keypair for the maker

  const [vault_state_pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("state"), signer.publicKey.toBuffer()],
    program.programId
  );

  const [vault_account] = PublicKey.findProgramAddressSync(
    [vault_state_pda.toBuffer()],
    program.programId
  );

  /*
  before("prepare", async () => {
    await airdrop(connection, signer.publicKey);
  })
  */
  
  console.log({signer:signer.publicKey.toString(), vault_state_pda:vault_state_pda.toString(), vault_account:vault_account.toString()});

  it("vault - initialize the vault!", async () => {
    // Airdrop SOL to the signer for transactions
    await airdrop(connection, signer.publicKey);

    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accountsPartial({
        signer: signer.publicKey,
        vaultState: vault_state_pda,
        vault: vault_account
      })
      .signers([signer])
      .rpc();
  });

  it("vault - deposit to the vault!", async () => {
    const vaultStateInfo = await connection.getAccountInfo(vault_state_pda);
    if (vaultStateInfo) {
      const vaultStateBalance = vaultStateInfo.lamports;
      console.log(
        `Vault State PDA Balance: ${vaultStateBalance} lamports (${vaultStateBalance / LAMPORTS_PER_SOL} SOL)`
      );
    } else {
      console.log("Vault State PDA account does not exist.");
    }

    let depositAmount = new BN(0.2 * LAMPORTS_PER_SOL);

    // Add your test here.
    const tx = await program.methods
      .deposit(depositAmount)
      .accountsPartial({
        signer: signer.publicKey,
        vaultState: vault_state_pda,
        vault: vault_account
      })
      .signers([signer])
      .rpc();

    // Vault account only is created after the deposit
    const vaultAccountInfo = await connection.getAccountInfo(vault_account);
    if (vaultAccountInfo) {
      const vaultAccountBalance = vaultAccountInfo.lamports;
      console.log(
        `Vault Account Balance: ${vaultAccountBalance} lamports (${vaultAccountBalance / LAMPORTS_PER_SOL} SOL)`
      );
    } else {
      console.log("Vault Account does not exist.");
    }
  });

  it("vault - withdraw from the vault!", async () => {
    let withdrawAmount = new BN(0.2 * LAMPORTS_PER_SOL);

    // Add your test here.
    const tx = await program.methods
      .withdraw(withdrawAmount)
      .accountsPartial({
        signer: signer.publicKey,
        vaultState: vault_state_pda,
        vault: vault_account
      })
      .signers([signer])
      .rpc();

    // Vault account only is created after the deposit
    const vaultAccountInfo = await connection.getAccountInfo(vault_account);
    if (vaultAccountInfo) {
      const vaultAccountBalance = vaultAccountInfo.lamports;
      console.log(
        `Vault Account Balance: ${vaultAccountBalance} lamports (${vaultAccountBalance / LAMPORTS_PER_SOL} SOL)`
      );
    } else {
      console.log("Vault Account does not exist.");
    }
  });


});

async function airdrop(connection: any, address: any, amount = LAMPORTS_PER_SOL) {
  const airdropTx = await connection.requestAirdrop(address, amount)
  await connection.confirmTransaction(airdropTx, "confirmed");
}
