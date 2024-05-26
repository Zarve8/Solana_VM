import {Keypair, PublicKey, Transaction} from "@solana/web3.js";
import {Provider} from "./provider";


export class Wallet {
    address: PublicKey;
    _privateKey: Keypair;
    private _provider: Provider;

    constructor(keypair: Keypair, provider: Provider) {
        this._privateKey = keypair;
        this.address = keypair.publicKey;
        this._provider = provider;
    }

    get publicKey(): PublicKey {
        return this.address;
    }

    async sendTransaction(tx: Transaction){
        await this._provider.sendTransaction(this, tx);
    }

    async getBalance() {
        return (await this._provider.connection.getBalance(this.address, {commitment: "confirmed"})).toString();
    }

    async requestAirdrop(){
        const airdropSignature  = await this._provider.connection.requestAirdrop(this.address, 1e9);
        await this._provider.connection.confirmTransaction(airdropSignature);
    }
}