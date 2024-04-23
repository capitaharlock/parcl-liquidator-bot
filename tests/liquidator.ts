import {
    ParclV3Sdk,
    getExchangePda,
    Address,
    MarginAccountWrapper,
  } from "@parcl-oss/v3-sdk";
  import {
    Commitment,
    Connection,
    Keypair,
    PublicKey,
  } from "@solana/web3.js";
  import bs58 from "bs58";
  import * as dotenv from "dotenv";
  dotenv.config();
  
  (async function main() {
    console.log("Loading margin accounts");

    // Env
    if (process.env.RPC_URL === undefined) {
      throw new Error("Missing RPC url");
    }
    if (process.env.PRIVATE_KEY === undefined) {
      throw new Error("Missing signer private key");
    }

    // Init
    const sdk = new ParclV3Sdk({ rpcUrl: process.env.RPC_URL });
    const connection = new Connection(process.env.RPC_URL, 'confirmed');
    const exchangeAddress = getExchangePda(0);
    const signer = Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY));
    const allMarginAccounts = await sdk.accountFetcher.getAllMarginAccounts();
    console.log(`Fetched ${allMarginAccounts.length} margin accounts`);

    // Process all accounts
    allMarginAccounts.forEach(account => {
      const marginAccount = new MarginAccountWrapper(account.account, account.address);
      console.log(`Account: ${marginAccount.address}`);
      
      // Set and filter CheckTimestamp 

      // Call liquidator program

    });

  })();