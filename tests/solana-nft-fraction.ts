import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaNftFraction } from "../target/types/solana_nft_fraction";
import { ComputeBudgetProgram, Keypair, PublicKey, SYSVAR_INSTRUCTIONS_PUBKEY, SystemProgram, Transaction, VersionedTransaction } from '@solana/web3.js';
import { Amount, Signer, TransactionBuilder, UmiError, generateRandomString, generateSigner, percentAmount, publicKey, publicKeyBytes, signTransaction, signerPayer, transactionBuilder } from '@metaplex-foundation/umi'
import {
  createV1,
  fetchDigitalAsset,
  fetchDigitalAssetWithTokenByMint,
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
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccount, createMint, getAssociatedTokenAddress } from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

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
  }).sendAndConfirm(umi, {
    send: {
      preflightCommitment: "confirmed",
    },
  })

  let createHash = base58.deserialize(createTx.signature);
  console.log("Created NFT metadata account", createHash)

  // Then mint the NFT to the authority
  let mintTx = await mintV1(umi, {
    mint: mint.publicKey,
    authority: umi.identity,
    amount: 1,
    tokenOwner: umi.identity.publicKey,
    tokenStandard: TokenStandard.NonFungible,
  }).sendAndConfirm(umi, {
    send: {
      preflightCommitment: "confirmed",
    },
  })

  let mint_hash = base58.deserialize(mintTx.signature);
  console.log("Minted NFT", mint_hash);

  return mint.publicKey
}

describe("solana-nft-fraction", () => {
  it("Creates nft and a fraction nft token", async () => {
    let nftMintStr = await createAndMintNft("MyFakeNft", "https://madlads.s3.us-west-2.amazonaws.com/json/5052.json")
    const digitalAsset = await fetchDigitalAssetWithTokenByMint(umi, publicKey(nftMintStr));

    const tokenMint = anchor.web3.Keypair.generate();

    const [nftVault, nftVaultBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("nft_vault")), tokenMint.publicKey.toBuffer()],
      program.programId
    );

    const [fractionPDA, fractionBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("fraction")), nftVault.toBuffer()],
      program.programId
    );

    const [fractionMetadataAccount, fractionMetadataAccountBump] = findMetadataPda(umi, {
      mint: publicKey(tokenMint.publicKey)
    });

    const ixArgs = {
      shareAmount: new anchor.BN(10),
      fractionAccount: fractionPDA,
    }

    let userTokenAccount = await getAssociatedTokenAddress(tokenMint.publicKey, provider.wallet.publicKey);

    const ixAccounts = {
      user: provider.wallet.publicKey,
      fractionAccount: fractionPDA,
      nftVault: nftVault,
      nftAccount: digitalAsset.token.publicKey,
      nftMint: digitalAsset.mint.publicKey,
      nftMetadataAccount: digitalAsset.metadata.publicKey,
      fractionTokenMetadata: fractionMetadataAccount,
      userTokenAccount: userTokenAccount,
      tokenMint: tokenMint.publicKey,
      tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      ataProgram: ASSOCIATED_PROGRAM_ID,
      sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      systemProgram: SystemProgram.programId,
    }
    let wallet = provider.wallet as anchor.Wallet;

    // We need to modify the compute units to be able to run the transaction
    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
      units: 1000000
    });

    let txid = await program.methods.fractionalizeNft(ixArgs.shareAmount)
      .accounts(ixAccounts)
      .signers([wallet.payer, tokenMint])
      .preInstructions([modifyComputeUnits])
      .rpc();

    // Log the tx id
    console.log("🎉 Transaction Succesfully Confirmed!");
    console.log("Transaction executed:", txid);
  });
});
