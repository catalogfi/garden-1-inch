import { Address, AuctionDetails } from '@1inch/fusion-sdk';
import { AuctionPoint, PresetData } from './types';
export declare class Preset {
    readonly auctionDuration: bigint;
    readonly startAuctionIn: bigint;
    readonly initialRateBump: number;
    readonly auctionStartAmount: bigint;
    readonly startAmount: bigint;
    readonly costInDstToken: bigint;
    readonly auctionEndAmount: bigint;
    readonly points: AuctionPoint[];
    readonly gasCostInfo: {
        gasBumpEstimate: bigint;
        gasPriceEstimate: bigint;
    };
    readonly exclusiveResolver?: Address;
    readonly allowPartialFills: boolean;
    readonly allowMultipleFills: boolean;
    readonly secretsCount: number;
    constructor(preset: PresetData);
    createAuctionDetails(additionalWaitPeriod?: bigint): AuctionDetails;
    private calcAuctionStartTime;
}
