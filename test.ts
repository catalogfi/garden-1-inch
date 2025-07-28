import axios from 'axios';
import fs from 'fs';
import path from 'path';
import Web3 from 'web3';
import { randomBytes } from 'node:crypto';
import { NetworkEnum } from '@1inch/cross-chain-sdk';

const privateKey = '0x7c4d215564e9ca136802d2e46f27ce548bff4bf1f394dd5d9c80b87b2501b649';
const rpc = 'https://base-mainnet.g.alchemy.com/v2/zN3JM2LnBeD4lFHMlO_iA8IoQA8Ws9_r';
const authKey = 'bz7ru1B8H8QFiFcDts1LfEUnjKNBJ5QK';
const source = 'FusionPlusBot';

const web3 = new Web3(rpc);
const walletAddress = web3.eth.accounts.privateKeyToAccount(privateKey).address;

const apiBaseUrl = 'https://api.1inch.dev/fusion-plus/quoter/v1.0';
const headers = {
    Authorization: `Bearer ${authKey}`,
    accept: 'application/json'
};

async function getQuote(): Promise<any> {
    const params = {
        amount: '1000000', // 1 USDC
        srcChain: NetworkEnum.COINBASE, // Base
        dstChain: NetworkEnum.ARBITRUM, // Arbitrum
        enableEstimate: true,
        srcTokenAddress: '0x833589fcd6edb6e08f4c7c32d4f71b54bda02913', // USDC on Base
        dstTokenAddress: '0xaf88d065e77c8cc2239327c5edb3a432268e5831', // USDC on Arbitrum
        walletAddress
    };

    try {
        console.log('Fetching quote with params:', params);
        const response = await axios.get(`${apiBaseUrl}/quote/receive`, {
            headers,
            params
        });
        const quote = response.data;
        console.log('Quote received:', quote);
        writeToJsonFile('quote', quote);
        return quote;
    } catch (error: any) {
        console.error('Error fetching quote:', error.response?.data || error.message);
        throw error;
    }
}

async function buildOrder(quote: any): Promise<{ hash: string; quoteId: string; order: any }> {
    const preset = 'fast';
    const secrets = Array.from({ length: quote.presets[preset].secretsCount }).map(
        () => '0x' + randomBytes(32).toString('hex')
    );
    const secretHashes = secrets.map((s) => {
        const hash = web3.utils.sha3(s);
        if (!hash) throw new Error('Failed to hash secret');
        return hash;
    });

    const payload = {
        amount: '1000000', // 1 USDC
        srcChain: NetworkEnum.COINBASE, // Base
        dstChain: NetworkEnum.ARBITRUM, // Arbitrum
        srcTokenAddress: '0x833589fcd6edb6e08f4c7c32d4f71b54bda02913', // USDC on Base
        dstTokenAddress: '0xaf88d065e77c8cc2239327c5edb3a432268e5831', // USDC on Arbitrum
        walletAddress: "0x3E53d785995bb74C0B9ba8F71D0d6a0c4d9E6901"
    };

    try {
        console.log('Building order with payload:', payload);
        const response = await axios.post(`${apiBaseUrl}/quote/build`, payload, { headers });
        const { hash, quoteId, order } = response.data;
        console.log('Order created, hash:', hash);
        writeToJsonFile('order', order);
        return { hash, quoteId, order };
    } catch (error: any) {
        console.error('Error building order:', error.response?.data || error.message);
        throw error;
    }
}

async function submitOrder(hash: string, quoteId: string, order: any, secretHashes: string[]): Promise<any> {
    const payload = {
        quoteId,
        order,
        secretHashes
    };

    try {
        console.log('Submitting order with hash:', hash);
        const response = await axios.post(`https://api.1inch.dev/fusion-plus/relayer/v1.0/submit`, payload, { headers });
        const orderInfo = response.data;
        console.log('Order submitted, hash:', hash);
        writeToJsonFile('order_submit', orderInfo);
        return orderInfo;
    } catch (error: any) {
        console.error('Error submitting order:', error.response?.data || error.message);
        throw error;
    }
}

async function main(): Promise<void> {
    try {
        const quote = await getQuote();

        const { hash, quoteId, order } = await buildOrder(quote);

        const secretHashes = order.secretHashes || []; // Fallback to empty array if undefined
        await submitOrder(hash, quoteId, order, secretHashes);
    } catch (error) {
        console.error('💥 Swap failed:', error);
        throw error;
    }
}

function writeToJsonFile(filename: string, data: any) {
    const dir = './responses';
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir);
    }
    const filePath = path.join(dir, `${filename}.json`);

    const replacer = (key: string, value: any) => {
        if (typeof value === 'bigint') {
            return value.toString() + 'n';
        }
        return value;
    };

    fs.writeFileSync(filePath, JSON.stringify(data, replacer, 2));
    console.log(`Response written to ${filePath}`);
}

main().catch((error) => {
    console.error('💀 Fatal error:', error);
    process.exit(1);
});

