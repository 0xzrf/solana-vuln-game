import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaVulnGame } from "../target/types/solana_vuln_game";
import {Keypair, PublicKey} from "@solana/web3.js" 
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("solana-vuln-game", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  
  const program = anchor.workspace.SolanaVulnGame as Program<SolanaVulnGame>;
  const owner = Keypair.generate()
  const config = PublicKey.findProgramAddressSync([Buffer.from("config")], program.programId)[0]
  const systemProgram = anchor.web3.SystemProgram.programId;
  const payer = Keypair.generate();
  const tokenProgram = TOKEN_PROGRAM_ID
  const tokenMint = PublicKey.findProgramAddressSync([Buffer.from("payment_token"), owner.publicKey.toBuffer(), config.toBuffer()],program.programId)[0]
  const user = PublicKey.findProgramAddressSync(
    [
      Buffer.from("user_account"),
      payer.publicKey.toBuffer()
    ],
    program.programId
  )[0]

  
  before(async () => {
    const sig_owner = await provider.connection.requestAirdrop(
      owner.publicKey,
      5 * anchor.web3.LAMPORTS_PER_SOL
    );
    await program.provider.connection.confirmTransaction(sig_owner);
    // funding the Payer and owner accounts with 5 sol each
    const sig_payer = await program.provider.connection.requestAirdrop(
      payer.publicKey,
      5 * anchor.web3.LAMPORTS_PER_SOL
    );
    await program.provider.connection.confirmTransaction(sig_payer);

    await program.methods.initializeConfig()
    .accountsStrict({
      admin: owner.publicKey,
      config,
      systemProgram,
      tokenProgram,
      tokenMint: tokenMint
    })


    await program.methods.initializeUser()
    .accountsStrict({
      systemProgram,
      user: payer.publicKey,
      userAccount: user
    })
  })
  

  it("Is initialized!", async () => {
    
      


  });
});
