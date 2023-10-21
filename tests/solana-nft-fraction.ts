import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaNftFraction } from "../target/types/solana_nft_fraction";
import { Keypair, SystemProgram } from '@solana/web3.js';
import { Amount, Signer, UmiError, generateSigner, percentAmount, publicKey, signerPayer, transactionBuilder } from '@metaplex-foundation/umi'
import {
  createV1,
  mintV1,
  mplTokenMetadata,
  TokenStandard,
} from '@metaplex-foundation/mpl-token-metadata'
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import { base58 } from "@metaplex-foundation/umi/serializers";

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
// Our smart contract
const program = anchor.workspace.SolanaNftFraction as Program<SolanaNftFraction>;

const signer = provider.wallet;
const umi = createUmi("https://api.devnet.solana.com")
  .use(walletAdapterIdentity(signer))
  .use(mplTokenMetadata());


const createAndMintNft = async (name: string, uri: string) => {
  // We generate a new Mint for the NFT.
  const mint = generateSigner(umi)

  // First create the metadata account
  let create_tx = await createV1(umi, {
    mint,
    authority: umi.identity,
    payer: umi.payer,
    name,
    uri,
    sellerFeeBasisPoints: percentAmount(0),
    tokenStandard: TokenStandard.NonFungible,
  }).sendAndConfirm(umi)

  let create_hash = base58.deserialize(create_tx.signature);
  console.log("Created NFT metadata account", create_hash)

  // Then mint the NFT to the authority
  let mint_tx = await mintV1(umi, {
    mint: mint.publicKey,
    authority: umi.identity,
    amount: 1,
    tokenOwner: umi.identity.publicKey,
    tokenStandard: TokenStandard.NonFungible,
  }).sendAndConfirm(umi)

  let mint_hash = base58.deserialize(mint_tx.signature);
  console.log("Minted NFT", mint_hash);
}

describe("solana-nft-fraction", () => {
  // Configure the client to use the local cluster.
  createAndMintNft("TestNft", "https://lh3.googleusercontent.com/22KjKODGuOPpyD9YgHnZpWPbt1-IhiEpPTkSbjHIa5sUjeUmzdG3UiO_dy1UKEUf4Iqc7kG5uBhW5JKYofyVGU4GUeApdsqplmo")

});
