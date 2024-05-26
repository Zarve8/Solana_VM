import {Connection, Keypair, sendAndConfirmTransaction, Transaction, VersionedTransaction} from "@solana/web3.js";
import Config from "./config";
import {Provider} from "./provider";
import {Wallet} from "./wallet";


interface SuperMessage {
    accounts: string[],
    instructions: SuperInstruction[],
    signers: number[],
    writable: number[],
    payer: number
}

interface SuperInstruction {
    data: number[],
    accounts: number[],
    program_id: []
}


let txStack: StackedTx[] = [];
let signatureStack: string[] = [];
interface StackedTx {
    tx: Transaction,
    signers: Keypair[]
}


export class SuperProvider extends Provider {
    constructor() {
        //@ts-ignore
        super(new Connection(Config.httpUrl, {fetch: SuperProvider.fetchFunc}));
    }

    static async fetchFunc(url: string, data: object) {
        let switch_res = true;
        while (switch_res) {
            try { //@ts-ignore
                const body = JSON.parse(data.body) //@ts-ignore
                // console.log(body.method)
                if (body.method == 'sendTransaction') {
                    //@ts-ignore
                    const res =  await SuperProvider.sendSuperTransaction(await SuperProvider.parseTx(body.params[0]));
                    signatureStack.push(res);
                    return undefined;
                }
                if (body.method == 'getLatestBlockhash') {
                    // console.log(await (await fetch(Config.superUrl+'/rpc', data)).json())
                    return await fetch(Config.superUrl+'/rpc', data);
                }
                if(body.method == 'getMinimumBalanceForRentExemption') {
                    return await fetch(Config.superUrl+'/rpc', data);
                }
                if (body.method == 'getAccountInfo') {
                    return await fetch(Config.superUrl+'/rpc', data);
                }
                if (body.method == 'getBalance') {
                    return await fetch(Config.superUrl+'/rpc', data);
                }
                console.log(await (await fetch(Config.httpUrl, data)).json())
                throw "Invalid Method";
                // return await fetch(Config.httpUrl, data);
            } catch (e) {
                throw e;
            }
        }
    }

    async sendTransaction(wallet: Wallet, tx: Transaction, signers: Keypair[] = []): Promise<string | undefined> {
        txStack.push({tx, signers: [wallet._privateKey].concat(signers)});
        try {
            let txId = await this.connection.sendTransaction(tx, [wallet._privateKey].concat(signers),{
                skipPreflight: false,
                preflightCommitment: "confirmed",
                //confirmation: "confirmed",
            });
            console.log(`https://explorer.solana.com/tx/${txId}?cluster=devnet`);
            return txId;
        }
        catch (e) {
            // console.log("Provider:SendTx", e);
            const signature = signatureStack[0];
            signatureStack = signatureStack.slice(1);
            return signature;
        }
    }

    static async sendSuperTransaction(tx: SuperMessage) {
        const response =  await fetch(Config.superUrl+'/super/send_transaction', {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(tx),
        });
        const res = await response.json()
        console.log("Super Transaction res:", JSON.stringify(res));
        if(response.status != 200) {
            console.log("Failed to Send", response);
        }
        return res.result;
    }

    static async parseTx(s: string) {
        const packedBuf = Uint8Array.from(Buffer.from(s, 'base64'));
        const tx = VersionedTransaction.deserialize(packedBuf);
        const stacked = txStack[0];
        txStack = txStack.slice(1);
        const accounts: string[] = tx.message.getAccountKeys().staticAccountKeys.map(key => key.toString());
        let msg = {
            accounts,
            instructions: tx.message.compiledInstructions.map(instr => {
                return {
                    program_id: instr.programIdIndex,
                    accounts: instr.accountKeyIndexes,
                    data: Array.from(Uint8Array.from(instr.data))
                }
            }),
            signers: stacked.signers.map(signer => {
                return accounts.indexOf(signer.publicKey.toString());
            }),
            writable: [...Array(accounts.length).keys()],
            payer: accounts.indexOf(stacked.tx.feePayer?.toBase58()!)
        }
        if(msg.payer < 0) msg.payer = msg.signers[0];
        // console.log("Parsed Message:", msg);
        return msg;
    }

    sendTransactionWithoutConfirmation(wallet: Wallet, tx: Transaction, signers?: Keypair[]): Promise<string | undefined> {
        return this.sendTransaction(wallet, tx, signers);
    }
}

/* Ok Response Body
{
  jsonrpc: '2.0',
  result: '4H97xbR6dwUYwqnPNehmRX6w7QM6qDxVdcvuNbc8xbxraHTGwTApdVSrxBSLAfXLyWEFNBicYpLUVD8EWCEtsy4U',
  id: '84943c04-7332-4a2d-924d-74ee248f481e'
}
 */


/*
{
    "accounts": ["FaGHKV74yrwsAgbp9SxadKhBbfAQteNbJEetsEWUppCa", "6yk2s4VtX1i7xN7R15WHGtU4AoMtStSBY98H87LtirW"],
    "instructions": [
        {
            "data": [0],
            "accounts": [],
            "program_id": 0
        }
    ],
    "signers": [1],
    "writable": [1],
    "payer": 1
}
 */