import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaNftFraction } from "../target/types/solana_nft_fraction";
import { Keypair, PublicKey, SYSVAR_INSTRUCTIONS_PUBKEY, SystemProgram, Transaction, VersionedTransaction } from '@solana/web3.js';
import { Amount, Signer, TransactionBuilder, UmiError, generateRandomString, generateSigner, percentAmount, publicKey, publicKeyBytes, signTransaction, signerPayer, transactionBuilder } from '@metaplex-foundation/umi'
import {
  createV1,
  fetchDigitalAsset,
  findMetadataPda,
  getMplTokenMetadataProgramId,
  mintV1,
  MPL_TOKEN_METADATA_PROGRAM_ID,
  mplTokenMetadata,
  TokenStandard,
} from '@metaplex-foundation/mpl-token-metadata'
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import { base58 } from "@metaplex-foundation/umi/serializers";
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccount, createMint } from "@solana/spl-token";

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

// Our smart contract
const program = anchor.workspace.SolanaNftFraction as Program<SolanaNftFraction>;

const signer = provider.wallet;
const umi = createUmi("https://api.devnet.solana.com")
  .use(walletAdapterIdentity(signer))
  .use(mplTokenMetadata());


const createAndMintNft = async (name: string, uri: string) => {
  // Our Nft Mint
  const mint = generateSigner(umi)

  // // First create the metadata account
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

  return mint.publicKey
}

describe("solana-nft-fraction", () => {
  // it("Create and mint NFT", async () => {
  //   await createAndMintNft("TestNFT", "https://www.stockphotosecrets.com/wp-content/uploads/2018/08/hide-the-pain-stockphoto-840x560.jpg")
  // });

  it("Creates nft and a fraction nft token", async () => {
    const digitalAsset = await fetchDigitalAsset(umi, publicKey("7Y7pLihtSvwFVCkrXKCnwu5nv31gYK4uNmBENfjiT6wu"));

    const [fractionPDA, fractionBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("fraction")), publicKeyBytes(digitalAsset.mint.publicKey)],
      program.programId
    );

    const [nftVault, nftVaultBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("nft_vault")), publicKeyBytes(digitalAsset.mint.publicKey)],
      program.programId
    );

    const tokenMint = anchor.web3.Keypair.generate();

    const [fractionMetadataAccount, fractionMetadataAccountBump] = findMetadataPda(umi, {
      mint: publicKey(tokenMint.publicKey)
    });

    const ixArgs = {
      shareAmount: new anchor.BN(10),
      fractionAccount: fractionPDA,
    }

    console.log("nftAccount: ", digitalAsset.publicKey);
    console.log("nftMint: ", digitalAsset.mint.publicKey);
    console.log("nftMetadataAccount: ", digitalAsset.metadata.publicKey);

    const ixAccounts = {
      user: provider.wallet.publicKey,
      fractionAccount: fractionPDA,
      nftVault: nftVault,
      nftAccount: digitalAsset.publicKey,
      nftMint: digitalAsset.mint.publicKey,
      nftMetadataAccount: digitalAsset.metadata.publicKey,
      fractionTokenMetadata: fractionMetadataAccount,
      tokenMint: tokenMint.publicKey, // This is the issue
      tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      systemProgram: SystemProgram.programId,
    }

    // This is good mainly for testing however we want to log the steps below
    let latest_blockhash = await provider.connection.getLatestBlockhash("confirmed");
    let wallet = provider.wallet as anchor.Wallet;
    let txid = await program.methods
      .fractionalizeNft(ixArgs.shareAmount)
      .accounts(ixAccounts)
      .signers([wallet.payer, tokenMint])
      .rpc();

    // Log the tx id
    console.log("🎉 Transaction Succesfully Confirmed!");
    console.log("Transaction executed:", txid);
  });
});
