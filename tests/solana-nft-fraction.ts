import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaNftFraction } from "../target/types/solana_nft_fraction";
import { TOKEN_PROGRAM_ID, MintLayout } from '@solana/spl-token';
import { Keypair, SystemProgram } from '@solana/web3.js';

describe("solana-nft-fraction", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaNftFraction as Program<SolanaNftFraction>;
  let mint = Keypair.generate();
  let user = anchor.web3.Keypair.generate();
  
  it("Is initialized!", async () => {
  });
});
