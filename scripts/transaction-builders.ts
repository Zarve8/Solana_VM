import { Keypair, Transaction, SystemProgram, Connection, PublicKey } from "@solana/web3.js";
import {
    ACCOUNT_SIZE,
    createInitializeAccountInstruction,
    getMinimumBalanceForRentExemptAccount,
    createInitializeMintInstruction,
    getMinimumBalanceForRentExemptMint,
    createMintToCheckedInstruction,
    createTransferCheckedInstruction,
    MINT_SIZE,
    TOKEN_PROGRAM_ID, createTransferInstruction, createMintToInstruction,
} from "@solana/spl-token";
import {Provider} from "./lib/provider";
import {Wallet} from "./lib/wallet";


export async function constructSolTransfer(provider: Provider, wallet: Wallet, to: PublicKey): Promise<[Transaction]>  {
    let tx = new Transaction();
    tx.add(
        SystemProgram.transfer({
            fromPubkey: wallet.publicKey,
            toPubkey: wallet.publicKey,
            lamports: 1
        })
    );
    return [tx];
}

export async function constructMintCreate(provider: Provider, wallet: Wallet): Promise<[Transaction, Keypair]>  {
    const mintAccount = Keypair.generate();
    console.log(`Random Mint Address: ${mintAccount.publicKey.toBase58()}`);
    let tx = new Transaction();
    tx.add(
        // create account
        SystemProgram.createAccount({
            fromPubkey: wallet.publicKey,
            newAccountPubkey: mintAccount.publicKey,
            space: MINT_SIZE,
            lamports: await getMinimumBalanceForRentExemptMint(provider.connection),
            programId: TOKEN_PROGRAM_ID,
        }),
        // init mint
        createInitializeMintInstruction(
            mintAccount.publicKey,
            0,
            wallet.publicKey,
            null
        )
    );
    return [tx, mintAccount];
}

export async function constructAccountCreate(provider: Provider, wallet: Wallet, mintAccount: Keypair): Promise<[Transaction, Keypair]> {
    let tokenAccount = Keypair.generate();
    console.log(`Random Token Address: ${tokenAccount.publicKey.toBase58()}`);
    let tx = new Transaction();
    tx.add(
        // create account
        SystemProgram.createAccount({
            fromPubkey: wallet.publicKey,
            newAccountPubkey: tokenAccount.publicKey,
            space: ACCOUNT_SIZE,
            lamports: await getMinimumBalanceForRentExemptAccount(provider.connection),
            programId: TOKEN_PROGRAM_ID,
        }),
        // init token account
        createInitializeAccountInstruction(tokenAccount.publicKey, mintAccount.publicKey, wallet.publicKey)
    );
    return [tx, tokenAccount];
}

export async function constructMintTo(provider: Provider, wallet: Wallet, mintAccount: Keypair, tokenAccount: Keypair): Promise<[Transaction]> {
    let tx = new Transaction();
    tx.add(
        createMintToInstruction(
            mintAccount.publicKey,
            tokenAccount.publicKey,
            wallet.publicKey,
            1,
        )
    );
    return [tx];
}

export async function constructTokenTransfer(provider: Provider, wallet: Wallet, mintAccount: Keypair, tokenAccount: Keypair, tokenAccount2: Keypair): Promise<[Transaction]> {
    let tx = new Transaction();
    tx.add(
        createTransferInstruction(
            tokenAccount.publicKey, // from
            tokenAccount2.publicKey,
            wallet.publicKey,
            1
        )
    );
    return [tx];
}
