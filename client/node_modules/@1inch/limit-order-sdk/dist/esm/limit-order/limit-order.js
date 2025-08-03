import { AbiCoder } from 'ethers';
import { isHexString, UINT_160_MAX, UINT_256_MAX } from '@1inch/byte-utils';
import assert from 'assert';
import { buildOrderTypedData, getLimitOrderV4Domain, getOrderHash } from './eip712';
import { MakerTraits } from './maker-traits';
import { Extension } from './extensions/extension';
import { injectTrackCode } from './source-track';
import { Address } from '../address';
import { randBigInt } from '../utils/rand-bigint';
export class LimitOrder {
    constructor(orderInfo, makerTraits = new MakerTraits(0n), extension = Extension.default()) {
        this.extension = extension;
        assert(!orderInfo.takerAsset.isNative(), `${orderInfo.takerAsset} can not be 'takerAsset'. Use wrapper address as 'takerAsset' and 'makerTraits.enableNativeUnwrap' to swap to NATIVE currency`);
        assert(!orderInfo.makerAsset.isNative(), 'Maker asset can not be NATIVE, use wrapper');
        this.makerAsset = orderInfo.makerAsset;
        this.takerAsset = orderInfo.takerAsset;
        this.makingAmount = orderInfo.makingAmount;
        this.takingAmount = orderInfo.takingAmount;
        this._salt = LimitOrder.verifySalt(orderInfo.salt || LimitOrder.buildSalt(extension), extension);
        this.maker = orderInfo.maker;
        this.receiver = orderInfo.receiver?.equal(orderInfo.maker)
            ? Address.ZERO_ADDRESS
            : orderInfo.receiver || Address.ZERO_ADDRESS;
        this.makerTraits = makerTraits;
        assert(this.makingAmount <= UINT_256_MAX, 'makingAmount too big');
        assert(this.takingAmount <= UINT_256_MAX, 'takingAmount too big');
        if (!extension.isEmpty()) {
            this.makerTraits.withExtension();
        }
    }
    get salt() {
        return this._salt;
    }
    static buildSalt(extension, baseSalt = randBigInt((1n << 96n) - 1n)) {
        if (extension.isEmpty()) {
            return baseSalt;
        }
        return (baseSalt << 160n) | (extension.keccak256() & UINT_160_MAX);
    }
    static verifySalt(salt, extension) {
        assert(salt <= UINT_256_MAX, 'salt too big');
        if (extension.isEmpty()) {
            return salt;
        }
        const hash = salt & UINT_160_MAX;
        const expectedHash = extension.keccak256() & UINT_160_MAX;
        assert(hash === expectedHash, 'invalid salt: lowest 160 bits should be extension hash');
        return salt;
    }
    static fromCalldata(bytes) {
        assert(isHexString(bytes), 'Bytes should be valid hex string with 0x prefix');
        const info = AbiCoder.defaultAbiCoder().decode([LimitOrder.Web3Type], bytes);
        const order = info[0];
        return new LimitOrder({
            salt: order.salt ? BigInt(order.salt) : undefined,
            maker: new Address(order.maker),
            receiver: new Address(order.receiver),
            takingAmount: BigInt(order.takingAmount),
            makingAmount: BigInt(order.makingAmount),
            takerAsset: new Address(order.takerAsset),
            makerAsset: new Address(order.makerAsset)
        }, new MakerTraits(BigInt(order.makerTraits)));
    }
    static fromDataAndExtension(data, extension) {
        return new LimitOrder({
            salt: BigInt(data.salt),
            maker: new Address(data.maker),
            receiver: new Address(data.receiver),
            takingAmount: BigInt(data.takingAmount),
            makingAmount: BigInt(data.makingAmount),
            takerAsset: new Address(data.takerAsset),
            makerAsset: new Address(data.makerAsset)
        }, new MakerTraits(BigInt(data.makerTraits)), extension);
    }
    setSource(source) {
        this._salt = injectTrackCode(this.salt, source);
        return this;
    }
    toCalldata() {
        return AbiCoder.defaultAbiCoder().encode([LimitOrder.Web3Type], [this.build()]);
    }
    build() {
        return {
            maker: this.maker.toString(),
            makerAsset: this.makerAsset.toString(),
            takerAsset: this.takerAsset.toString(),
            makerTraits: (this.makerTraits?.asBigInt() || 0n).toString(),
            salt: this.salt.toString(),
            makingAmount: this.makingAmount.toString(),
            takingAmount: this.takingAmount.toString(),
            receiver: this.receiver.toString()
        };
    }
    getTypedData(chainId) {
        const domain = getLimitOrderV4Domain(chainId);
        return buildOrderTypedData(domain.chainId, domain.verifyingContract, domain.name, domain.version, this.build());
    }
    getOrderHash(chainId) {
        return getOrderHash(this.getTypedData(chainId));
    }
    isPrivate() {
        return this.makerTraits.isPrivate();
    }
}
LimitOrder.Web3Type = `tuple(${[
    'uint256 salt',
    'address maker',
    'address receiver',
    'address makerAsset',
    'address takerAsset',
    'uint256 makingAmount',
    'uint256 takingAmount',
    'uint256 makerTraits'
]})`;
//# sourceMappingURL=limit-order.js.map