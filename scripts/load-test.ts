import {SuperProvider} from "./lib/super-provider";
import {Wallet} from "./lib/wallet";
import Config from "./lib/config";
import {requestAccountBalance, requestAccountInfo} from "./request-builder";
import {SolanaProvider} from "./lib/solana-provider";
import {Provider} from "./lib/provider";
import {PublicKey} from "@solana/web3.js";
import {constructMintCreate, constructSolTransfer} from "./transaction-builders";


async function checkResponseTime(envName: string, requestName: string, func: () => Promise<void>): Promise<number> {
    let time1 = performance.now();
    await func();
    let time2 = performance.now();
    const delta = time2 - time1;
    console.log(`${envName}: ${requestName} finished in ${delta}sec`)
    return delta;
}

async function checkAverageResponseTime(envName: string, requestName: string, func: () => Promise<void>, num: number): Promise<number> {
    console.log("");
    let a: number[] = [];
    for(let i: number = 0; i < num; i++) {
        a.push(await checkResponseTime(envName, requestName, func));
    }
    const average = (array: number[]) => {return array.reduce((a, b) => a + b) / array.length};
    let avg = average(a);
    console.log(`AVG: ${avg}`);
    console.log("");
    return avg;
}

async function displayLoad(envName: string, testNames: string[], funcs: (() => Promise<void>)[]) {
    console.log(`========= ${envName} Load ========`);
    let values: number[] = [];
    for(let i: number = 0; i < funcs.length; i++) {
        values.push(await checkAverageResponseTime(
            envName,
            testNames[i],
            funcs[i],
            10
        ));
    }
    const max = (array: string[]) => {return array.reduce((a, b) => a.length>b.length? a: b).length};
    console.log("======== Average ========");
    const maxNameLength = max(testNames);
    for(let i: number = 0; i < funcs.length; i++) {
        const delimiter = new Array(maxNameLength - testNames.length).join( ' ');
        console.log(`${testNames[i]}:${delimiter}   ${values[i]}`);
    }
    console.log("======== End ========\n");
}

async function loadProvider(bSol: boolean) {
    let envName = bSol? "Sol": "VM";
    let provider: Provider = bSol? new SolanaProvider(): new SuperProvider();
    const wallet = new Wallet(Config.walletSecret, provider);

    const getBalanceTest = async() => {
        await requestAccountBalance(provider, wallet.publicKey);
    };

    const getAccountTest = async() => {
        await requestAccountInfo(provider, new PublicKey("HHmabb6Jz2e5jUR1vVuQQi5onZbwmEVJ42qsbKrZ7xhh"));
    };

    const SendSolTest = async () => {
        const [tx] = await constructSolTransfer(provider, wallet, new PublicKey("HHmabb6Jz2e5jUR1vVuQQi5onZbwmEVJ42qsbKrZ7xhh"));
        await provider.sendTransaction(wallet, tx, []);
    };

    const CreateMintTest = async () => {
      const [tx, mintAccount] = await constructMintCreate(provider, wallet);
        await provider.sendTransaction(wallet, tx, [mintAccount]);
    };

    await displayLoad(envName,
        ["getBalance", "getAccountInfo", "Sol Transfer TX", "Mint Creation TX"],
        [getBalanceTest, getAccountTest, SendSolTest, CreateMintTest]);
}


//HHmabb6Jz2e5jUR1vVuQQi5onZbwmEVJ42qsbKrZ7xhh
async function main() {
    await loadProvider(false);
}


main();

