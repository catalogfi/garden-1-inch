"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.LimitOrder = void 0;
const tslib_1 = require("tslib");
const ethers_1 = require("ethers");
const byte_utils_1 = require("@1inch/byte-utils");
const assert_1 = tslib_1.__importDefault(require("assert"));
const eip712_1 = require("./eip712");
const maker_traits_1 = require("./maker-traits");
const extension_1 = require("./extensions/extension");
const source_track_1 = require("./source-track");
const address_1 = require("../address");
const rand_bigint_1 = require("../utils/rand-bigint");
class LimitOrder {
    constructor(orderInfo, makerTraits = new maker_traits_1.MakerTraits(0n), extension = extension_1.Extension.default()) {
        this.extension = extension;
        (0, assert_1.default)(!orderInfo.takerAsset.isNative(), `${orderInfo.takerAsset} can not be 'takerAsset'. Use wrapper address as 'takerAsset' and 'makerTraits.enableNativeUnwrap' to swap to NATIVE currency`);
        (0, assert_1.default)(!orderInfo.makerAsset.isNative(), 'Maker asset can not be NATIVE, use wrapper');
        this.makerAsset = orderInfo.makerAsset;
        this.takerAsset = orderInfo.takerAsset;
        this.makingAmount = orderInfo.makingAmount;
        this.takingAmount = orderInfo.takingAmount;
        this._salt = LimitOrder.verifySalt(orderInfo.salt || LimitOrder.buildSalt(extension), extension);
        this.maker = orderInfo.maker;
        this.receiver = orderInfo.receiver?.equal(orderInfo.maker)
            ? address_1.Address.ZERO_ADDRESS
            : orderInfo.receiver || address_1.Address.ZERO_ADDRESS;
        this.makerTraits = makerTraits;
        (0, assert_1.default)(this.makingAmount <= byte_utils_1.UINT_256_MAX, 'makingAmount too big');
        (0, assert_1.default)(this.takingAmount <= byte_utils_1.UINT_256_MAX, 'takingAmount too big');
        if (!extension.isEmpty()) {
            this.makerTraits.withExtension();
        }
    }
    get salt() {
        return this._salt;
    }
    static buildSalt(extension, baseSalt = (0, rand_bigint_1.randBigInt)((1n << 96n) - 1n)) {
        if (extension.isEmpty()) {
            return baseSalt;
        }
        return (baseSalt << 160n) | (extension.keccak256() & byte_utils_1.UINT_160_MAX);
    }
    static verifySalt(salt, extension) {
        (0, assert_1.default)(salt <= byte_utils_1.UINT_256_MAX, 'salt too big');
        if (extension.isEmpty()) {
            return salt;
        }
        const hash = salt & byte_utils_1.UINT_160_MAX;
        const expectedHash = extension.keccak256() & byte_utils_1.UINT_160_MAX;
        (0, assert_1.default)(hash === expectedHash, 'invalid salt: lowest 160 bits should be extension hash');
        return salt;
    }
    static fromCalldata(bytes) {
        (0, assert_1.default)((0, byte_utils_1.isHexString)(bytes), 'Bytes should be valid hex string with 0x prefix');
        const info = ethers_1.AbiCoder.defaultAbiCoder().decode([LimitOrder.Web3Type], bytes);
        const order = info[0];
        return new LimitOrder({
            salt: order.salt ? BigInt(order.salt) : undefined,
            maker: new address_1.Address(order.maker),
            receiver: new address_1.Address(order.receiver),
            takingAmount: BigInt(order.takingAmount),
            makingAmount: BigInt(order.makingAmount),
            takerAsset: new address_1.Address(order.takerAsset),
            makerAsset: new address_1.Address(order.makerAsset)
        }, new maker_traits_1.MakerTraits(BigInt(order.makerTraits)));
    }
    static fromDataAndExtension(data, extension) {
        return new LimitOrder({
            salt: BigInt(data.salt),
            maker: new address_1.Address(data.maker),
            receiver: new address_1.Address(data.receiver),
            takingAmount: BigInt(data.takingAmount),
            makingAmount: BigInt(data.makingAmount),
            takerAsset: new address_1.Address(data.takerAsset),
            makerAsset: new address_1.Address(data.makerAsset)
        }, new maker_traits_1.MakerTraits(BigInt(data.makerTraits)), extension);
    }
    setSource(source) {
        this._salt = (0, source_track_1.injectTrackCode)(this.salt, source);
        return this;
    }
    toCalldata() {
        return ethers_1.AbiCoder.defaultAbiCoder().encode([LimitOrder.Web3Type], [this.build()]);
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
        const domain = (0, eip712_1.getLimitOrderV4Domain)(chainId);
        return (0, eip712_1.buildOrderTypedData)(domain.chainId, domain.verifyingContract, domain.name, domain.version, this.build());
    }
    getOrderHash(chainId) {
        return (0, eip712_1.getOrderHash)(this.getTypedData(chainId));
    }
    isPrivate() {
        return this.makerTraits.isPrivate();
    }
}
exports.LimitOrder = LimitOrder;
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