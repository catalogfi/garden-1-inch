"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Immutables = void 0;
const tslib_1 = require("tslib");
const fusion_sdk_1 = require("@1inch/fusion-sdk");
const ethers_1 = require("ethers");
const byte_utils_1 = require("@1inch/byte-utils");
const assert_1 = tslib_1.__importDefault(require("assert"));
const hash_lock_1 = require("../cross-chain-order/hash-lock");
const time_locks_1 = require("../cross-chain-order/time-locks");
/**
 * Contains escrow params for both source and destination chains
 * Determinate addresses of escrow contracts
 */
class Immutables {
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
            this.token = fusion_sdk_1.Address.NATIVE_CURRENCY;
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
        (0, assert_1.default)((0, byte_utils_1.isHexBytes)(bytes));
        const res = ethers_1.AbiCoder.defaultAbiCoder().decode([Immutables.Web3Type], bytes);
        const data = res.at(0);
        return new Immutables(data.orderHash, hash_lock_1.HashLock.fromString(data.hashlock), new fusion_sdk_1.Address(data.maker), new fusion_sdk_1.Address(data.taker), new fusion_sdk_1.Address(data.token), BigInt(data.amount), BigInt(data.safetyDeposit), time_locks_1.TimeLocks.fromBigInt(BigInt(data.timelocks)));
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
            timeLocks: time_locks_1.TimeLocks.fromBigInt(this.timeLocks.build()).setDeployedAt(time)
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
        return (0, ethers_1.keccak256)(this.encode());
    }
    build() {
        const token = this.token.isNative() ? fusion_sdk_1.Address.ZERO_ADDRESS : this.token;
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
        return ethers_1.AbiCoder.defaultAbiCoder().encode([Immutables.Web3Type], [this.build()]);
    }
}
exports.Immutables = Immutables;
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