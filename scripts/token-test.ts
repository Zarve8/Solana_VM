import { Keypair, Transaction, SystemProgram, Connection, PublicKey } from "@solana/web3.js";
import {
    ACCOUNT_SIZE,
    createAssociatedTokenAccountInstruction,
    createInitializeAccountInstruction,
    getAssociatedTokenAddress,
    getMinimumBalanceForRentExemptAccount,
    createInitializeMintInstruction,
    getMinimumBalanceForRentExemptMint,
    createMintToCheckedInstruction,
    createTransferCheckedInstruction,
    MINT_SIZE,
    TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {Provider} from "./lib/provider";
import {Wallet} from "./lib/wallet";
import Config from "./lib/config";
import {SuperProvider} from "./lib/super-provider";
import {
    constructAccountCreate,
    constructMintCreate,
    constructMintTo,
    constructTokenTransfer
} from "./transaction-builders";
import {SolanaProvider} from "./lib/solana-provider";


async function compareAccount(p1: Provider, p2: Provider, account: PublicKey) {
    const buf1 = await p1.getAccountData(account);
    const buf2 = await p2.getAccountData(account);
    if(buf1.equals(buf2)) {
        console.log(`Account Match: ${account.toString()}`);
    }
    else {
        console.log(`Account not match: ${account.toString()}`);
        console.log(JSON.stringify(buf1));
        console.log(JSON.stringify(buf2));
    }

}


async function main() {
    const provider = new SuperProvider();
    const solProvider = new SolanaProvider();
    const wallet = new Wallet(Config.walletSecret, provider);

    console.log("======== Create Mint Account =========");
    const [createMintTx, mintAccount] = await constructMintCreate(provider, wallet); //@ts-ignore
    await provider.sendTransaction(wallet, createMintTx, [mintAccount]); //@ts-ignore
    await solProvider.sendTransaction(wallet, createMintTx, [mintAccount]); //@ts-ignore
    await compareAccount(provider, solProvider, mintAccount.publicKey);

    console.log("\n\n======== Create Token Account1 =========");
    //@ts-ignore
    const [createAccountTx, tokenAccount] = await constructAccountCreate(provider, wallet, mintAccount); //@ts-ignore
    await provider.sendTransaction(wallet, createAccountTx, [tokenAccount]); //@ts-ignore
    await solProvider.sendTransaction(wallet, createAccountTx, [tokenAccount]); //@ts-ignore
    await compareAccount(provider, solProvider, tokenAccount.publicKey);

    console.log("\n\n======== Create Token Account2 =========");
    //@ts-ignore
    const [createAccount2Tx, tokenAccount2] = await constructAccountCreate(provider, wallet, mintAccount); //@ts-ignore
    await provider.sendTransaction(wallet, createAccount2Tx, [tokenAccount2]); //@ts-ignore
    await solProvider.sendTransaction(wallet, createAccount2Tx, [tokenAccount2]); //@ts-ignore
    await compareAccount(provider, solProvider, tokenAccount2.publicKey);

    console.log("\n\n======== Mint To Account1 =========");
    await solProvider.sendTransaction(wallet,
        (await constructMintTo(solProvider, wallet, mintAccount, tokenAccount))[0], []); //@ts-ignore
    await provider.sendTransaction(wallet,
        (await constructMintTo(provider, wallet, mintAccount, tokenAccount))[0], []);
    await compareAccount(provider, solProvider, tokenAccount.publicKey); //@ts-ignore
    await compareAccount(provider, solProvider, mintAccount.publicKey);

    console.log("\n\n======== Token Transfer =========");
    //@ts-ignore
    const [transferTx] = await constructTokenTransfer(provider, wallet, mintAccount, tokenAccount, tokenAccount2);
    await provider.sendTransaction(wallet, transferTx, []); //@ts-ignore
    await solProvider.sendTransaction(wallet, transferTx, []); //@ts-ignore
    await compareAccount(provider, solProvider, tokenAccount.publicKey);
    await compareAccount(provider, solProvider, tokenAccount2.publicKey);
}

main();
