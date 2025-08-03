import { Extension } from './extensions/extension';
import { Interaction } from './interaction';
import { Address } from '../address';
export declare enum AmountMode {
    taker = 0,
    maker = 1
}
export declare class TakerTraits {
    private static MAKER_AMOUNT_FLAG;
    private static UNWRAP_WETH_FLAG;
    private static SKIP_ORDER_PERMIT_FLAG;
    private static USE_PERMIT2_FLAG;
    private static ARGS_HAS_RECEIVER;
    private static THRESHOLD_MASK;
    private static ARGS_INTERACTION_LENGTH_MASK;
    private static ARGS_EXTENSION_LENGTH_MASK;
    private flags;
    private receiver?;
    private extension?;
    private interaction?;
    constructor(flag: bigint, data: {
        receiver?: Address;
        extension?: Extension;
        interaction?: Interaction;
    });
    static default(): TakerTraits;
    getAmountMode(): AmountMode;
    setAmountMode(mode: AmountMode): this;
    isNativeUnwrapEnabled(): boolean;
    enableNativeUnwrap(): this;
    disableNativeUnwrap(): this;
    isOrderPermitSkipped(): boolean;
    skipOrderPermit(): this;
    isPermit2Enabled(): boolean;
    enablePermit2(): this;
    disablePermit2(): this;
    setReceiver(receiver: Address): this;
    removeReceiver(): this;
    setExtension(ext: Extension): this;
    removeExtension(): this;
    setAmountThreshold(threshold: bigint): this;
    getAmountThreshold(): bigint;
    removeAmountThreshold(): this;
    setInteraction(interaction: Interaction): this;
    removeInteraction(): this;
    encode(): {
        trait: bigint;
        args: string;
    };
}
