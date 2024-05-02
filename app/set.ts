import * as anchor from '@project-serum/anchor';
import { PublicKey, SystemProgram, Keypair } from '@solana/web3.js';
import { readFileSync } from 'fs';
import { homedir } from 'os';
import { Parcl as ParclProgram } from '../target/types/parcl';
import { ParclV3Sdk, getMarginAccountPda, Position} from '@parcl-oss/v3-sdk';
import BN from "bn.js";


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
async function testMarginAccount() {

    // Simulate user wallet
    let userWallet = new Keypair();
    console.log('User Wallet Public Key:', userWallet.publicKey.toString());

    // Init data
    const ownerPublicKey = provider.wallet.publicKey;
    const exchangePublicKey = new PublicKey('82dGS7Jt4Km8ZgwZVRsJ2V6vPXEhVdgDaMP7cqTTTTTT'); // PublicKey | 82dGS7Jt4Km8ZgwZVRsJ2V6vPXEhVdgDaMP7cqPGG1TW
    const delegate = new PublicKey('5ZM92feLzB2okid74vVfGRzPSindNfNHwfNrEGh81UG1');          // PublicKey
    const padding = new Uint8Array(10); // [u8; 10]
    const id: number = 1;               // u32
    let positions: any[];               // Array (Position)
    let margin: BN;                     // u64
    let maxLiquidationFee: BN;          // u64
    let inLiquidation: number;          // u8
    
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
    /*
    console.log('positions:', typeof positions, positions);
    console.log('margin:', typeof margin, margin.toString());
    console.log('maxLiquidationFee:', typeof maxLiquidationFee, maxLiquidationFee.toString());
    console.log('id:', typeof id, id);
    console.log('exchange:', typeof exchangePublicKey, exchangePublicKey.toString());
    console.log('delegate:', typeof delegate, delegate.toString());
    console.log('inLiquidation:', typeof inLiquidation, inLiquidation);
    console.log('padding:', typeof padding, padding);
    */

    // *** Initiate account
    console.log('\nInitiating Account');

    // set data
    positions = [];
    margin = new BN("3000");
    maxLiquidationFee = new BN("5000");
    inLiquidation = 0;

    // send transaction
    try {
        const tx = await program.rpc.setMarginAccount(
            id,
            positions,
            margin,
            maxLiquidationFee,
            exchangePublicKey,
            delegate,
            inLiquidation,
            {
            accounts: {
                marginAccount: marginAccount,
                exchange: exchangePublicKey,
                user: ownerPublicKey,
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

    // *** Modify account data
    console.log('\nModifying Account');

    // set data
    positions = [];
    margin = new BN("3000");
    maxLiquidationFee = new BN("5000");
    inLiquidation = 0;

    // send transaction
    try {
        const tx = await program.rpc.putMarginAccount(
            id,
            positions,
            margin,
            maxLiquidationFee,
            inLiquidation,
            {
            accounts: {
                marginAccount: marginAccount,
                exchange: exchangePublicKey,
                user: ownerPublicKey,
                systemProgram: SystemProgram.programId,
            },
            signers: [walletKeyPair],
        });

        console.log('Transaction signature:', tx);
        await connection.confirmTransaction(tx, 'confirmed');
        console.log('Transaction confirmed');
    } catch (error) {
        console.error('Error during the transaction:', error);
    }

}

// *** Test requests
testMarginAccount().catch(console.error);