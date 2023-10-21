import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaNftFraction } from "../target/types/solana_nft_fraction";
import { Keypair, SYSVAR_INSTRUCTIONS_PUBKEY, SystemProgram } from '@solana/web3.js';
import { Amount, Signer, UmiError, generateRandomString, generateSigner, percentAmount, publicKey, publicKeyBytes, signerPayer, transactionBuilder } from '@metaplex-foundation/umi'
import {
  createV1,
  fetchDigitalAsset,
  findMetadataPda,
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
    // mint: mint.publicKey,
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
  it("Creates nft and a fraction nft token", async () => {
    let nftMint = await createAndMintNft("TestNFT", "https://www.stockphotosecrets.com/wp-content/uploads/2018/08/hide-the-pain-stockphoto-840x560.jpg")

    const digitalAsset = await fetchDigitalAsset(umi, publicKey(nftMint));

    const [fractionPDA, fractionBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("fraction")), publicKeyBytes(digitalAsset.mint.publicKey)],
      program.programId
    );

    const [nftVault, nftVaultBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("nft_vault")), publicKeyBytes(digitalAsset.mint.publicKey)],
      program.programId
    );

    const tokenMint = await generateSigner(umi);
    const [fractionMetadataAccount, fractionMetadataAccountBump] = findMetadataPda(umi, {
      mint: tokenMint.publicKey
    });

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
      fractionTokenMetadata: fractionMetadataAccount,
      tokenMint: tokenMint.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      systemProgram: SystemProgram.programId,
    }

    let transaction = await program.methods.fractionalizeNft(ixArgs.shareAmount).accounts(ixAccounts).rpc();
    console.log("Successfully fractionalized NFT ", transaction);
  });
});
