"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FeeTakerExtension = void 0;
const tslib_1 = require("tslib");
const byte_utils_1 = require("@1inch/byte-utils");
const assert_1 = tslib_1.__importDefault(require("assert"));
const fees_1 = require("./fees");
const resolver_fee_1 = require("./resolver-fee");
const integrator_fee_1 = require("./integrator-fee");
const whitelist_half_address_1 = require("./whitelist-half-address");
const fee_calculator_1 = require("./fee-calculator");
const extension_builder_1 = require("../extension-builder");
const address_1 = require("../../../address");
const interaction_1 = require("../../interaction");
const extension_1 = require("../extension");
const bps_1 = require("../../../bps");
class FeeTakerExtension {
    constructor(address, fees, whitelist, makerPermit, extraInteraction, customReceiver) {
        this.address = address;
        this.fees = fees;
        this.whitelist = whitelist;
        this.makerPermit = makerPermit;
        this.extraInteraction = extraInteraction;
        this.customReceiver = customReceiver;
    }
    static new(address, fees, whitelist, extra) {
        return new FeeTakerExtension(address, fees, whitelist_half_address_1.WhitelistHalfAddress.new(whitelist || []), extra?.makerPermit, extra?.extraInteraction, extra?.customReceiver);
    }
    static decode(bytes) {
        const extension = extension_1.Extension.decode(bytes);
        return FeeTakerExtension.fromExtension(extension);
    }
    static fromExtension(extension) {
        const extensionAddress = address_1.Address.fromFirstBytes(extension.makingAmountData);
        (0, assert_1.default)(address_1.Address.fromFirstBytes(extension.takingAmountData).equal(extensionAddress) &&
            address_1.Address.fromFirstBytes(extension.postInteraction).equal(extensionAddress), 'Invalid extension, all calls should be to the same address');
        (0, assert_1.default)(extension.takingAmountData == extension.makingAmountData, 'Invalid extension, taking amount data must be equal to making amount data');
        const interactionBytes = byte_utils_1.BytesIter.HexString(extension.postInteraction);
        interactionBytes.nextUint160();
        const flags = byte_utils_1.BN.fromHex(interactionBytes.nextUint8());
        const integratorFeeRecipient = new address_1.Address(interactionBytes.nextUint160());
        const protocolFeeRecipient = new address_1.Address(interactionBytes.nextUint160());
        const customTokensRecipient = flags.getBit(FeeTakerExtension.CUSTOM_RECEIVER_FLAG_BIT)
            ? new address_1.Address(interactionBytes.nextUint160())
            : undefined;
        const interactionData = parseAmountData(interactionBytes);
        const extraInteraction = interactionBytes.isEmpty()
            ? undefined
            : interaction_1.Interaction.decode(interactionBytes.rest());
        const amountBytes = byte_utils_1.BytesIter.HexString(extension.makingAmountData);
        amountBytes.nextUint160();
        const amountData = parseAmountData(amountBytes);
        const permit = extension.hasMakerPermit
            ? interaction_1.Interaction.decode(extension.makerPermit)
            : undefined;
        (0, assert_1.default)(amountData.fees.integratorFee.value ===
            interactionData.fees.integratorFee.value, `invalid extension: integrator fee must be same in interaction data and in amount data`);
        (0, assert_1.default)(amountData.fees.resolverFee.value ===
            interactionData.fees.resolverFee.value, `invalid extension: resolver fee must be same in interaction data and in amount data`);
        (0, assert_1.default)(amountData.whitelist.discount.value ===
            interactionData.whitelist.discount.value, `invalid extension: whitelist discount fee must be same in interaction data and in amount data`);
        (0, assert_1.default)(amountData.fees.integratorShare.value ===
            interactionData.fees.integratorShare.value, `invalid extension: integrator share must be same in interaction data and in amount data`);
        (0, assert_1.default)(interactionData.whitelist.addresses.length ===
            amountData.whitelist.addresses.length, 'whitelist must be same in interaction data and in amount data');
        (0, assert_1.default)(interactionData.whitelist.addresses.every((val, i) => amountData.whitelist.addresses[i] === val), 'whitelist must be same in interaction data and in amount data');
        return new FeeTakerExtension(extensionAddress, new fees_1.Fees(amountData.fees.resolverFee.isZero()
            ? resolver_fee_1.ResolverFee.ZERO
            : new resolver_fee_1.ResolverFee(protocolFeeRecipient, amountData.fees.resolverFee, amountData.whitelist.discount), amountData.fees.integratorFee.isZero()
            ? integrator_fee_1.IntegratorFee.ZERO
            : new integrator_fee_1.IntegratorFee(integratorFeeRecipient, protocolFeeRecipient, amountData.fees.integratorFee, amountData.fees.integratorShare)), new whitelist_half_address_1.WhitelistHalfAddress(amountData.whitelist.addresses), permit, extraInteraction, customTokensRecipient);
    }
    getFeeCalculator() {
        return new fee_calculator_1.FeeCalculator(this.fees, this.whitelist);
    }
    build() {
        const amountGetterData = this.buildAmountGetterData();
        const builder = new extension_builder_1.ExtensionBuilder()
            .withMakingAmountData(this.address, amountGetterData)
            .withTakingAmountData(this.address, amountGetterData)
            .withPostInteraction(new interaction_1.Interaction(this.address, this.buildInteractionData()));
        if (this.makerPermit) {
            builder.withMakerPermit(this.makerPermit.target, this.makerPermit.data);
        }
        return builder.build();
    }
    getTakingAmount(taker, takingAmount) {
        return this.getFeeCalculator().getTakingAmount(taker, takingAmount);
    }
    getMakingAmount(taker, makingAmount) {
        return this.getFeeCalculator().getMakingAmount(taker, makingAmount);
    }
    getResolverFee(taker, takingAmount) {
        return this.getFeeCalculator().getResolverFee(taker, takingAmount);
    }
    getIntegratorFee(taker, takingAmount) {
        return this.getFeeCalculator().getIntegratorFee(taker, takingAmount);
    }
    getProtocolShareOfIntegratorFee(taker, takingAmount) {
        return this.getFeeCalculator().getProtocolShareOfIntegratorFee(taker, takingAmount);
    }
    getProtocolFee(taker, takingAmount) {
        return this.getFeeCalculator().getProtocolFee(taker, takingAmount);
    }
    buildAmountGetterData() {
        const integrator = {
            fee: this.fees.integrator.fee.toFraction(fees_1.Fees.BASE_1E5),
            share: this.fees.integrator.share.toFraction(fees_1.Fees.BASE_1E2)
        };
        const resolverFee = this.fees.resolver.fee.toFraction(fees_1.Fees.BASE_1E5);
        const builder = new byte_utils_1.BytesBuilder()
            .addUint16(BigInt(integrator.fee))
            .addUint8(BigInt(integrator.share))
            .addUint16(BigInt(resolverFee))
            .addUint8(BigInt(Number(fees_1.Fees.BASE_1E2) -
            this.fees.resolver.whitelistDiscount.toFraction(fees_1.Fees.BASE_1E2)));
        this.whitelist.encodeTo(builder);
        return builder.asHex();
    }
    buildInteractionData() {
        const flags = new byte_utils_1.BN(0n).setBit(FeeTakerExtension.CUSTOM_RECEIVER_FLAG_BIT, Boolean(this.customReceiver));
        const builder = new byte_utils_1.BytesBuilder()
            .addUint8(flags)
            .addAddress(this.fees.integrator.integrator.toString())
            .addAddress(this.fees.protocol.toString());
        if (this.customReceiver) {
            builder.addAddress(this.customReceiver.toString());
        }
        builder.addBytes(this.buildAmountGetterData());
        if (this.extraInteraction) {
            builder
                .addAddress(this.extraInteraction.target.toString())
                .addBytes(this.extraInteraction.data);
        }
        return builder.asHex();
    }
}
exports.FeeTakerExtension = FeeTakerExtension;
FeeTakerExtension.CUSTOM_RECEIVER_FLAG_BIT = 0n;
function parseAmountData(iter) {
    const fees = {
        integratorFee: bps_1.Bps.fromFraction(Number(iter.nextUint16()), fees_1.Fees.BASE_1E5),
        integratorShare: bps_1.Bps.fromFraction(Number(iter.nextUint8()), fees_1.Fees.BASE_1E2),
        resolverFee: bps_1.Bps.fromFraction(Number(iter.nextUint16()), fees_1.Fees.BASE_1E5)
    };
    const whitelistDiscount = bps_1.Bps.fromFraction(Number(fees_1.Fees.BASE_1E2) - Number(iter.nextUint8()), fees_1.Fees.BASE_1E2);
    const whitelistAddresses = [];
    const whitelistFromAmountSize = Number(iter.nextUint8());
    for (let i = 0; i < whitelistFromAmountSize; i++) {
        whitelistAddresses.push(iter.nextBytes(10));
    }
    return {
        fees,
        whitelist: { discount: whitelistDiscount, addresses: whitelistAddresses }
    };
}
//# sourceMappingURL=fee-taker.extension.js.map