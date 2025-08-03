export type PaginationParams = {
    page?: number;
    limit?: number;
};
export declare class PaginationRequest {
    page: number | undefined;
    limit: number | undefined;
    constructor(page: number | undefined, limit: number | undefined);
    validate(): string | null;
}
