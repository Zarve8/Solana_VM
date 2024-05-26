import {Connection, Keypair, Transaction} from "@solana/web3.js";
import Config from "./config";
import {Wallet} from "./wallet";
import {Provider} from "./provider";

export class SolanaProvider extends Provider {
    constructor() {
        super(new Connection(Config.httpUrl));
    }

    async sendTransactionWithoutConfirmation(wallet: Wallet, tx: Transaction, signers: Keypair[] = []): Promise<string | undefined> {
        try {
            let txId = await this.connection.sendTransaction(tx, [wallet._privateKey].concat(signers), {
                skipPreflight: false,
                preflightCommitment: "confirmed",
            });
            console.log(`https://explorer.solana.com/tx/${txId}?cluster=devnet`);
            return txId;
        } catch (e) {
            console.log("Provider:SendTx", e);
            return undefined;
        }
    }
}