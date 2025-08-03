export class WhitelistHalfAddress {
    constructor(addresses) {
        this.addresses = addresses;
    }
    get length() {
        return this.addresses.length;
    }
    static new(addresses) {
        return new WhitelistHalfAddress(addresses?.map((w) => w.lastHalf()) || []);
    }
    isWhitelisted(address) {
        const half = address.lastHalf();
        return this.addresses.some((w) => w === half);
    }
    encodeTo(builder) {
        builder.addUint8(BigInt(this.addresses.length));
        for (const halfAddress of this.addresses) {
            builder.addBytes(halfAddress);
        }
        return builder;
    }
}
//# sourceMappingURL=whitelist-half-address.js.map