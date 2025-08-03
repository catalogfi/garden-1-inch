import { Address } from '@1inch/limit-order-sdk';
import { AuctionPoint, PresetData } from './types';
import { AuctionDetails } from '../../fusion-order';
export declare class Preset {
    readonly auctionDuration: bigint;
    readonly startAuctionIn: bigint;
    readonly bankFee: bigint;
    readonly initialRateBump: number;
    readonly auctionStartAmount: bigint;
    readonly auctionEndAmount: bigint;
    readonly tokenFee: bigint;
    readonly points: AuctionPoint[];
    readonly gasCostInfo: {
        gasBumpEstimate: bigint;
        gasPriceEstimate: bigint;
    };
    readonly exclusiveResolver?: Address;
    readonly allowPartialFills: boolean;
    readonly allowMultipleFills: boolean;
    constructor(preset: PresetData);
    createAuctionDetails(additionalWaitPeriod?: bigint): AuctionDetails;
    private calcAuctionStartTime;
}
