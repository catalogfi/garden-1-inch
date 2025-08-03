import { Address } from '@1inch/fusion-sdk';
import { AbiCoder, keccak256 } from 'ethers';
import { isHexBytes } from '@1inch/byte-utils';
import assert from 'assert';
import { HashLock } from '../cross-chain-order/hash-lock';
import { TimeLocks } from '../cross-chain-order/time-locks';
/**
 * Contains escrow params for both source and destination chains
 * Determinate addresses of escrow contracts
 */
export class Immutables {
    constructor(orderHash, hashLock, maker, 
    /**
     * Address who can withdraw funds, also to this address funds will be transferred in case of public withdrawal
     */
    taker, token, amount, safetyDeposit, timeLocks) {
        this.orderHash = orderHash;
        this.hashLock = hashLock;
        this.maker = maker;
        this.taker = taker;
        this.token = token;
        this.amount = amount;
        this.safetyDeposit = safetyDeposit;
        this.timeLocks = timeLocks;
        if (this.token.isZero()) {
            this.token = Address.NATIVE_CURRENCY;
        }
    }
    static new(params) {
        return new Immutables(params.orderHash, params.hashLock, params.maker, params.taker, params.token, params.amount, params.safetyDeposit, params.timeLocks);
    }
    /**
     * Create instance from encoded bytes
     * @param bytes 0x prefixed hex string
     */
    static decode(bytes) {
        assert(isHexBytes(bytes));
        const res = AbiCoder.defaultAbiCoder().decode([Immutables.Web3Type], bytes);
        const data = res.at(0);
        return new Immutables(data.orderHash, HashLock.fromString(data.hashlock), new Address(data.maker), new Address(data.taker), new Address(data.token), BigInt(data.amount), BigInt(data.safetyDeposit), TimeLocks.fromBigInt(BigInt(data.timelocks)));
    }
    toJSON() {
        return this.build();
    }
    withComplement(dstComplement) {
        return Immutables.new({ ...this, ...dstComplement });
    }
    withDeployedAt(time) {
        return Immutables.new({
            ...this,
            timeLocks: TimeLocks.fromBigInt(this.timeLocks.build()).setDeployedAt(time)
        });
    }
    withTaker(taker) {
        return Immutables.new({ ...this, taker });
    }
    withHashLock(hashLock) {
        return Immutables.new({ ...this, hashLock });
    }
    withAmount(amount) {
        return Immutables.new({ ...this, amount });
    }
    /**
     * Return keccak256 hash of instance
     */
    hash() {
        return keccak256(this.encode());
    }
    build() {
        const token = this.token.isNative() ? Address.ZERO_ADDRESS : this.token;
        return {
            orderHash: this.orderHash,
            hashlock: this.hashLock.toString(),
            maker: this.maker.toString(),
            taker: this.taker.toString(),
            token: token.toString(),
            amount: this.amount.toString(),
            safetyDeposit: this.safetyDeposit.toString(),
            timelocks: this.timeLocks.build().toString()
        };
    }
    /**
     * Encode instance as bytes
     */
    encode() {
        return AbiCoder.defaultAbiCoder().encode([Immutables.Web3Type], [this.build()]);
    }
}
Immutables.Web3Type = `tuple(${[
    'bytes32 orderHash',
    'bytes32 hashlock',
    'address maker',
    'address taker',
    'address token',
    'uint256 amount',
    'uint256 safetyDeposit',
    'uint256 timelocks'
]})`;
//# sourceMappingURL=immutables.js.map