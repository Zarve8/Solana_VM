import {Provider} from "./lib/provider";
import {PublicKey} from "@solana/web3.js";


export async function requestPing(provider: Provider) {

}

export async function requestAccountBalance(provider: Provider, account: PublicKey) {
    await provider.connection.getBalance(account, {commitment: 'confirmed'});
}

export async function requestAccountInfo(provider: Provider, account: PublicKey) {
    await provider.getAccountData(account);
}