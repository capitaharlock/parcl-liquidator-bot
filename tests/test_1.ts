import * as anchor from '@project-serum/anchor';
import { PublicKey, SystemProgram, Keypair } from '@solana/web3.js';
import { readFileSync } from 'fs';
import { homedir } from 'os';
import { Parcl as ParclProgram }  from '../target/types/parcl';

// Load local keypair
function loadKeypair(path: string): Keypair {
    const secretKeyString = readFileSync(path, { encoding: 'utf8' });
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}

// Set up the provider and program (IDL)
const idlPath = `${__dirname}/../target/idl/parcl.json`;
const idlJson = JSON.parse(readFileSync(idlPath, 'utf8'));
const clusterUrl = process.env.CLUSTER_URL || anchor.web3.clusterApiUrl('devnet');
const connection = new anchor.web3.Connection(clusterUrl);
const walletKeyPair = loadKeypair(`${homedir()}/.config/solana/id.json`);
const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(walletKeyPair), { preflightCommitment: 'confirmed' });
anchor.setProvider(provider);
const program = new anchor.Program<ParclProgram>(idlJson, idlJson.metadata.address, provider);

// Clone a margin account
async function cloneMarginAccount(sourceAccountPubkey: string, seed: string) {
    const sourceAccount = new PublicKey(sourceAccountPubkey);

    console.log('Initiating RPC call to clone margin account...');
    try {
        const tx = await program.rpc.cloneMarginAccount(seed, {
            accounts: {
                sourceAccount: sourceAccount,
                user: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            },
            signers: [],
        });
        console.log('Transaction signature:', tx);
        await connection.confirmTransaction(tx, 'confirmed');
        console.log('Transaction confirmed');
    } catch (error) {
        console.error('Error during the transaction:', error);
    }
}

// Execute
cloneMarginAccount('8SDrSBBoAUBKMLNxwVCzkrwXbSh75UyxavpDssxVhkB4', 'cloned_seed').catch(console.error);
