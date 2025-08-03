import { CustomPreset, QuoterCustomPresetRequestParams } from './types';
export declare class QuoterCustomPresetRequest {
    readonly customPreset: CustomPreset;
    constructor(params: QuoterCustomPresetRequestParams);
    static new(params: QuoterCustomPresetRequestParams): QuoterCustomPresetRequest;
    build(): CustomPreset;
    validate(): string | null;
    private validateAuctionDuration;
    private validatePoints;
}
