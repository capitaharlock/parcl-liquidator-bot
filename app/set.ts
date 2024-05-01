import * as anchor from '@project-serum/anchor';
import { PublicKey, SystemProgram, Keypair } from '@solana/web3.js';
import { readFileSync } from 'fs';
import { homedir } from 'os';
import { Parcl as ParclProgram } from '../target/types/parcl';
import { ParclV3Sdk, getMarginAccountPda, Position} from '@parcl-oss/v3-sdk';
import BN from "bn.js";
import { Decimal } from 'decimal.js';


// Load local keypair
function loadKeypair(path: string): Keypair {
    const secretKeyString = readFileSync(path, { encoding: 'utf8' });
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}

// Set up the provider and program (IDL)
const idlPath = `${__dirname}/../target/idl/parcl.json`;
const idlJson = JSON.parse(readFileSync(idlPath, 'utf8'));
const rpcUrl = process.env.CLUSTER_URL || anchor.web3.clusterApiUrl('devnet');
const connection = new anchor.web3.Connection(rpcUrl);
const walletKeyPair = loadKeypair(`${homedir()}/.config/solana/id.json`);
const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(walletKeyPair), { preflightCommitment: 'confirmed' });
anchor.setProvider(provider);
const program = new anchor.Program<ParclProgram>(idlJson, idlJson.metadata.address, provider);

// Parcl
const sdk = new ParclV3Sdk({ rpcUrl });

// Function to simulate Positions and other data
async function setMarginAccount() {
    console.log('Initiating RPC call');

    // Simulate user wallet
    let userWallet = new Keypair();
    console.log('User Wallet Public Key:', userWallet.publicKey.toString());

    // Test data
    const positions = [];
    const ownerPublicKey = provider.wallet.publicKey;
    const margin = new BN("0");  // u64
    const maxLiquidationFee = new BN("5000");  // u64
    const id = 1;     // u32
    const exchangePublicKey = new PublicKey('82dGS7Jt4Km8ZgwZVRsJ2V6vPXEhVdgDaMP7cqPGG1TW');
    const delegate = new PublicKey('5ZM92feLzB2okid74vVfGRzPSindNfNHwfNrEGh81UG1');
    const inLiquidation = 0;  // u8
    const padding = new Uint8Array(10);  // Ensure it's exactly 10 bytes
    
    /*
    // Margin account PDA | userWallet.publicKey (parcl sdk)
    const [marginAccount] = getMarginAccountPda(exchangePublicKey, ownerPublicKey, id);
    console.log("marginAccount: ", marginAccount);
    */

    const seeds = [
        Buffer.from("margin_account"),
        exchangePublicKey.toBuffer(),
        ownerPublicKey.toBuffer(),
        new Uint8Array(new Uint32Array([id]).buffer)
    ];
    const [marginAccount, bump] = await PublicKey.findProgramAddress(
        seeds,
        program.programId
    );

    // Log
    console.log('positions:', typeof positions, positions);
    console.log('margin:', typeof margin, margin.toString());
    console.log('maxLiquidationFee:', typeof maxLiquidationFee, maxLiquidationFee.toString());
    console.log('id:', typeof id, id);
    console.log('exchange:', typeof exchangePublicKey, exchangePublicKey.toString());
    console.log('delegate:', typeof delegate, delegate.toString());
    console.log('inLiquidation:', typeof inLiquidation, inLiquidation);
    console.log('padding:', typeof padding, padding);

    // Initiate transaction
    try {
        const tx = await program.rpc.setMarginAccount(
            id,
            positions, 
            {
            accounts: {
                marginAccount: marginAccount,
                exchange: exchangePublicKey,
                owner: ownerPublicKey,
                systemProgram: SystemProgram.programId,
            },
            signers: [walletKeyPair], // User wallet
        });

        console.log('Transaction signature:', tx);
        await connection.confirmTransaction(tx, 'confirmed');
        console.log('Transaction confirmed');
    } catch (error) {
        console.error('Error during the transaction:', error);
    }

}

// End User create account request
setMarginAccount().catch(console.error);