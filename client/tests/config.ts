import { z } from 'zod'
import Sdk from '@1inch/cross-chain-sdk'
import * as process from 'node:process'

const bool = z
    .string()
    .transform((v) => v.toLowerCase() === 'true')
    .pipe(z.boolean())

const ConfigSchema = z.object({
    SRC_CHAIN_RPC: z.string().url(),
    DST_CHAIN_RPC: z.string().url(),
    SRC_CHAIN_CREATE_FORK: bool.default('true'),
    DST_CHAIN_CREATE_FORK: bool.default('true')
})

const fromEnv = ConfigSchema.parse(process.env)
/* 

  = Logs ==
  TokenUSDC deployed at: 0x19eAC199abcc6f8dDe59198fcA5d44513B519368
  TokenWETH deployed at: 0xB0E39d4745eCe7C843834956e11cb16d68157be9
  LimitOrderProtocol deployed at: 0xf850CF9A70Fe8279F49739F1A14528D8BCe675e2

*/
export const config = {
    chain: {
        destination: {
            chainId: 84532,
            // url: "https://base-sepolia.g.alchemy.com/v2/0XPjrbBAKRJaSJuy6GN8uKX5uy7YquZV",
            url: fromEnv.SRC_CHAIN_RPC,
            // createFork: fromEnv.SRC_CHAIN_CREATE_FORK,
            // limitOrderProtocol: '0x4B715df6F89624dDb2c6DB70304b494d79531d92', // my deploy
            limitOrderProtocol: '0xf850CF9A70Fe8279F49739F1A14528D8BCe675e2', // my deploy2
            wrappedNative: '0xB0E39d4745eCe7C843834956e11cb16d68157be9', // my deployed
            ownerPrivateKey: '0x639ed7560cbdde79096973912f5c83de86ba08aef2ce6f673dad5bf0a1663801', // my private key 
            tokens: {
                USDC: {
                    address: '0x19eAC199abcc6f8dDe59198fcA5d44513B519368', // erc20 mock deployed
                    donor: '0x1B150538E943F00127929f7eeB65754f7beB0B6d'
                }
            }
        },
        
        source: {
            chainId: 10143,
            // url: "https://monad-testnet.g.alchemy.com/v2/0XPjrbBAKRJaSJuy6GN8uKX5uy7YquZV",
            url: fromEnv.DST_CHAIN_RPC,
            // createFork: fromEnv.DST_CHAIN_CREATE_FORK,
            limitOrderProtocol: '0xf850CF9A70Fe8279F49739F1A14528D8BCe675e2', // my deployed
            wrappedNative: '0xB0E39d4745eCe7C843834956e11cb16d68157be9', // our deployed 
            ownerPrivateKey: '0x639ed7560cbdde79096973912f5c83de86ba08aef2ce6f673dad5bf0a1663801',
            tokens: {
                USDC: {
                    address: '0x19eAC199abcc6f8dDe59198fcA5d44513B519368', // same native
                    donor: '0x1B150538E943F00127929f7eeB65754f7beB0B6d' // 
                }
            }
        }
    }
} as const

export type ChainConfig = (typeof config.chain)['source' | 'destination']

// {
//             chainId: 128123,
//             // url: "https://rpc.ankr.com/etherlink_testnet",
//             url: fromEnv.SRC_CHAIN_RPC,
//             // createFork: fromEnv.SRC_CHAIN_CREATE_FORK,
//             // limitOrderProtocol: '0x4B715df6F89624dDb2c6DB70304b494d79531d92', // my deploy
//             limitOrderProtocol: '0x9f2C105C9b1843019836889539C9F6993D3aa239', // my deploy2
//             wrappedNative: '0x75DBDb60C37A9c776206F2e44D2098054222af65', // my deployed
//             ownerPrivateKey: '0x639ed7560cbdde79096973912f5c83de86ba08aef2ce6f673dad5bf0a1663801', // my private key 
//             tokens: {
//                 USDC: {
//                     address: '0x6756682b6144018deA5416640A0d0e8783e33F60', // erc20 mock deployed
//                     donor: '0x1B150538E943F00127929f7eeB65754f7beB0B6d'
//                 }
//             }
//         },

// to check allownace see addresses and rpc
// cast call 0xEA2bB31EBb0aee264aba3730C8744d6bD76D37d0 "allowance(address,address)" 0x1B150538E943F00127929f7eeB65754f7beB0B6d 0x1f1259C74c1b6aDe7f57f239F77530AEdC6542B9 --rpc-url https://rpc.ankr.com/monad_testnet --private-key 0x149bc17929e5d9c43fb25ab94c112803130137bfdb2a2cfd6ef9043bd4fc22d6