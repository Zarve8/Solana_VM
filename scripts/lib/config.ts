import {Keypair, PublicKey} from "@solana/web3.js";

namespace Config {
    export const SystemProgram = new PublicKey("11111111111111111111111111111111");
    export const TokenProgram = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    export const AssociatedProgram = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
    export const MetadataProgram = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    export const Sysvar = new PublicKey("SysvarRent111111111111111111111111111111111");

    export const httpUrl: string = "https://clean-cosmopolitan-leaf.solana-devnet.discover.quiknode.pro/a14dd84a03329f246004d56b0eac79b2b716193b/";
    export const superUrl: string = "http://localhost:8080";

    export const walletSecret = Keypair.fromSecretKey(new Uint8Array([9,121,37,12,49,157,114,78,179,195,44,123,36,186,162,27,216,61,40,167,22,103,42,108,235,172,90,130,177,196,30,218,80,247,130,50,210,73,58,239,231,177,3,109,113,232,19,49,230,54,34,156,3,13,30,205,151,81,46,195,243,240,111,165]))
    export const walletAddress = walletSecret.publicKey;


}


export default Config;