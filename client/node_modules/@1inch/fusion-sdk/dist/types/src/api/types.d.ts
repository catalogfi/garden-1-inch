import { NetworkEnum } from '../constants';
import { HttpProviderConnector } from '../connector';
export type FusionApiConfig = {
    url: string;
    network: NetworkEnum;
    authKey?: string;
    httpProvider?: HttpProviderConnector;
};
export type PaginationMeta = {
    totalItems: number;
    itemsPerPage: number;
    totalPages: number;
    currentPage: number;
};
export type PaginationOutput<T extends Record<string, any> = Record<string, any>> = {
    meta: PaginationMeta;
    items: T[];
};
