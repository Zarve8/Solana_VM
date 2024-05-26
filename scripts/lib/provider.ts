import {Connection, Keypair, PublicKey, sendAndConfirmTransaction, Transaction} from "@solana/web3.js";
import {Wallet} from "./wallet";


export abstract class Provider {
    connection: Connection;

    protected constructor(connection: Connection) {
        this.connection = connection;
    }

    async sendTransaction(wallet: Wallet, tx: Transaction, signers: Keypair[] = []): Promise<string | undefined> {
        try {
            let txId = await sendAndConfirmTransaction(this.connection, tx, [wallet._privateKey].concat(signers),{
                skipPreflight: false,
                preflightCommitment: "confirmed",
                //confirmation: "confirmed",
            });
            console.log(`https://explorer.solana.com/tx/${txId}?cluster=devnet`);
            return txId;
        }
        catch (e) {
            console.log("Provider:SendTx", e);
            return undefined;
        }
    }

    async getAccountData(account: PublicKey) {
        try {
            return (await this.connection.getAccountInfo(account, {commitment: "confirmed"}))!.data;
        }
        catch (e) {
            return Buffer.from([]);
        }
    }

    abstract sendTransactionWithoutConfirmation(wallet: Wallet, tx: Transaction, signers: Keypair[]): Promise<string | undefined>;
}


