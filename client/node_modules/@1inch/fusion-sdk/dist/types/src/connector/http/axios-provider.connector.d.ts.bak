import { HttpProviderConnector } from './http-provider.connector';
export declare class AxiosProviderConnector implements HttpProviderConnector {
    private readonly authKey?;
    constructor(authKey?: string | undefined);
    get<T>(url: string): Promise<T>;
    post<T>(url: string, data: unknown): Promise<T>;
}
