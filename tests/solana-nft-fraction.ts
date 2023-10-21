import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaNftFraction } from "../target/types/solana_nft_fraction";
import { Keypair, SYSVAR_INSTRUCTIONS_PUBKEY, SystemProgram } from '@solana/web3.js';
import { Amount, Signer, UmiError, generateRandomString, generateSigner, percentAmount, publicKey, publicKeyBytes, signerPayer, transactionBuilder } from '@metaplex-foundation/umi'
import {
  createV1,
  fetchDigitalAsset,
  mintV1,
  mplTokenMetadata,
  TokenStandard,
} from '@metaplex-foundation/mpl-token-metadata'
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import { base58 } from "@metaplex-foundation/umi/serializers";
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccount } from "@solana/spl-token";

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

// Our smart contract
const program = anchor.workspace.SolanaNftFraction as Program<SolanaNftFraction>;

const signer = provider.wallet;
const umi = createUmi("https://api.devnet.solana.com")
  .use(walletAdapterIdentity(signer))
  .use(mplTokenMetadata());

// Our Nft Mint
const mint = generateSigner(umi)

const createAndMintNft = async (name: string, uri: string) => {

  // First create the metadata account
  let createTx = await createV1(umi, {
    mint,
    authority: umi.identity,
    payer: umi.payer,
    name,
    uri,
    sellerFeeBasisPoints: percentAmount(0),
    tokenStandard: TokenStandard.NonFungible,
  }).sendAndConfirm(umi)

  let createHash = base58.deserialize(createTx.signature);
  console.log("Created NFT metadata account", createHash)

  // Then mint the NFT to the authority
  let mintTx = await mintV1(umi, {
    mint: mint.publicKey,
    authority: umi.identity,
    amount: 1,
    tokenOwner: umi.identity.publicKey,
    tokenStandard: TokenStandard.NonFungible,
  }).sendAndConfirm(umi)

  let mint_hash = base58.deserialize(mintTx.signature);
  console.log("Minted NFT", mint_hash);
}

describe("solana-nft-fraction", () => {
  // Configure the client to use the local cluster.
  test("Creates and mints an NFT", async () => {
    createAndMintNft("TestNft", "https://lh3.googleusercontent.com/22KjKODGuOPpyD9YgHnZpWPbt1-IhiEpPTkSbjHIa5sUjeUmzdG3UiO_dy1UKEUf4Iqc7kG5uBhW5JKYofyVGU4GUeApdsqplmo")
  });


  test("Creates a fraction nft token", async () => {
    const digitalAsset = await fetchDigitalAsset(umi, mint.publicKey);

    const [fractionPDA, fractionBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("fraction")), publicKeyBytes(digitalAsset.mint.publicKey)],
      program.programId
    );

    const [nftVault, nftVaultBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("nft_vault")), publicKeyBytes(digitalAsset.mint.publicKey)],
      program.programId
    );
    
    const tokenMint = await generateSigner(umi);

    const ixArgs = {
      shareAmount: new anchor.BN(10),
      fractionAccount: fractionPDA,
    }

    const ixAccounts = {
      user: umi.identity.publicKey,
      fractionAccount: fractionPDA,
      nftVault: nftVault,
      nftAccount: digitalAsset.publicKey,
      nftMint: digitalAsset.mint.publicKey,
      nftMetadataAccount: digitalAsset.metadata.publicKey,
      tokenMint: tokenMint.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      systemProgram: SystemProgram.programId,
    }

    program.methods.fractionalizeNft(ixArgs.shareAmount).accounts(ixAccounts)
  });
});
