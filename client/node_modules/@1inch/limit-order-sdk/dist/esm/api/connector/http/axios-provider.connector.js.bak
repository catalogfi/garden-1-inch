import axios, { isAxiosError } from 'axios';
import { AuthError } from '../../errors';
export class AxiosProviderConnector {
    async get(url, headers) {
        try {
            const res = await axios.get(url, {
                headers
            });
            return res.data;
        }
        catch (error) {
            if (isAxiosError(error) && error.response?.status === 401) {
                throw new AuthError();
            }
            throw error;
        }
    }
    async post(url, data, headers) {
        try {
            const res = await axios.post(url, data, {
                headers
            });
            return res.data;
        }
        catch (error) {
            if (isAxiosError(error) && error.response?.status === 401) {
                throw new AuthError();
            }
            throw error;
        }
    }
}
//# sourceMappingURL=axios-provider.connector.js.map