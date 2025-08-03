import { BN, BytesBuilder, BytesIter } from '@1inch/byte-utils';
import assert from 'assert';
import { Fees } from './fees';
import { ResolverFee } from './resolver-fee';
import { IntegratorFee } from './integrator-fee';
import { WhitelistHalfAddress } from './whitelist-half-address';
import { FeeCalculator } from './fee-calculator';
import { ExtensionBuilder } from '../extension-builder';
import { Address } from '../../../address';
import { Interaction } from '../../interaction';
import { Extension } from '../extension';
import { Bps } from '../../../bps';
export class FeeTakerExtension {
    constructor(address, fees, whitelist, makerPermit, extraInteraction, customReceiver) {
        this.address = address;
        this.fees = fees;
        this.whitelist = whitelist;
        this.makerPermit = makerPermit;
        this.extraInteraction = extraInteraction;
        this.customReceiver = customReceiver;
    }
    static new(address, fees, whitelist, extra) {
        return new FeeTakerExtension(address, fees, WhitelistHalfAddress.new(whitelist || []), extra?.makerPermit, extra?.extraInteraction, extra?.customReceiver);
    }
    static decode(bytes) {
        const extension = Extension.decode(bytes);
        return FeeTakerExtension.fromExtension(extension);
    }
    static fromExtension(extension) {
        const extensionAddress = Address.fromFirstBytes(extension.makingAmountData);
        assert(Address.fromFirstBytes(extension.takingAmountData).equal(extensionAddress) &&
            Address.fromFirstBytes(extension.postInteraction).equal(extensionAddress), 'Invalid extension, all calls should be to the same address');
        assert(extension.takingAmountData == extension.makingAmountData, 'Invalid extension, taking amount data must be equal to making amount data');
        const interactionBytes = BytesIter.HexString(extension.postInteraction);
        interactionBytes.nextUint160();
        const flags = BN.fromHex(interactionBytes.nextUint8());
        const integratorFeeRecipient = new Address(interactionBytes.nextUint160());
        const protocolFeeRecipient = new Address(interactionBytes.nextUint160());
        const customTokensRecipient = flags.getBit(FeeTakerExtension.CUSTOM_RECEIVER_FLAG_BIT)
            ? new Address(interactionBytes.nextUint160())
            : undefined;
        const interactionData = parseAmountData(interactionBytes);
        const extraInteraction = interactionBytes.isEmpty()
            ? undefined
            : Interaction.decode(interactionBytes.rest());
        const amountBytes = BytesIter.HexString(extension.makingAmountData);
        amountBytes.nextUint160();
        const amountData = parseAmountData(amountBytes);
        const permit = extension.hasMakerPermit
            ? Interaction.decode(extension.makerPermit)
            : undefined;
        assert(amountData.fees.integratorFee.value ===
            interactionData.fees.integratorFee.value, `invalid extension: integrator fee must be same in interaction data and in amount data`);
        assert(amountData.fees.resolverFee.value ===
            interactionData.fees.resolverFee.value, `invalid extension: resolver fee must be same in interaction data and in amount data`);
        assert(amountData.whitelist.discount.value ===
            interactionData.whitelist.discount.value, `invalid extension: whitelist discount fee must be same in interaction data and in amount data`);
        assert(amountData.fees.integratorShare.value ===
            interactionData.fees.integratorShare.value, `invalid extension: integrator share must be same in interaction data and in amount data`);
        assert(interactionData.whitelist.addresses.length ===
            amountData.whitelist.addresses.length, 'whitelist must be same in interaction data and in amount data');
        assert(interactionData.whitelist.addresses.every((val, i) => amountData.whitelist.addresses[i] === val), 'whitelist must be same in interaction data and in amount data');
        return new FeeTakerExtension(extensionAddress, new Fees(amountData.fees.resolverFee.isZero()
            ? ResolverFee.ZERO
            : new ResolverFee(protocolFeeRecipient, amountData.fees.resolverFee, amountData.whitelist.discount), amountData.fees.integratorFee.isZero()
            ? IntegratorFee.ZERO
            : new IntegratorFee(integratorFeeRecipient, protocolFeeRecipient, amountData.fees.integratorFee, amountData.fees.integratorShare)), new WhitelistHalfAddress(amountData.whitelist.addresses), permit, extraInteraction, customTokensRecipient);
    }
    getFeeCalculator() {
        return new FeeCalculator(this.fees, this.whitelist);
    }
    build() {
        const amountGetterData = this.buildAmountGetterData();
        const builder = new ExtensionBuilder()
            .withMakingAmountData(this.address, amountGetterData)
            .withTakingAmountData(this.address, amountGetterData)
            .withPostInteraction(new Interaction(this.address, this.buildInteractionData()));
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
            fee: this.fees.integrator.fee.toFraction(Fees.BASE_1E5),
            share: this.fees.integrator.share.toFraction(Fees.BASE_1E2)
        };
        const resolverFee = this.fees.resolver.fee.toFraction(Fees.BASE_1E5);
        const builder = new BytesBuilder()
            .addUint16(BigInt(integrator.fee))
            .addUint8(BigInt(integrator.share))
            .addUint16(BigInt(resolverFee))
            .addUint8(BigInt(Number(Fees.BASE_1E2) -
            this.fees.resolver.whitelistDiscount.toFraction(Fees.BASE_1E2)));
        this.whitelist.encodeTo(builder);
        return builder.asHex();
    }
    buildInteractionData() {
        const flags = new BN(0n).setBit(FeeTakerExtension.CUSTOM_RECEIVER_FLAG_BIT, Boolean(this.customReceiver));
        const builder = new BytesBuilder()
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
FeeTakerExtension.CUSTOM_RECEIVER_FLAG_BIT = 0n;
function parseAmountData(iter) {
    const fees = {
        integratorFee: Bps.fromFraction(Number(iter.nextUint16()), Fees.BASE_1E5),
        integratorShare: Bps.fromFraction(Number(iter.nextUint8()), Fees.BASE_1E2),
        resolverFee: Bps.fromFraction(Number(iter.nextUint16()), Fees.BASE_1E5)
    };
    const whitelistDiscount = Bps.fromFraction(Number(Fees.BASE_1E2) - Number(iter.nextUint8()), Fees.BASE_1E2);
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