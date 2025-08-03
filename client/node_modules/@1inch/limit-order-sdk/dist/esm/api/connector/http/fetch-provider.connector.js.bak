import { AuthError } from '../../errors';
export class FetchProviderConnector {
    async get(url, headers) {
        const res = await fetch(url, { headers, method: 'GET' });
        if (res.status === 401) {
            throw new AuthError();
        }
        if (res.ok) {
            return res.json();
        }
        throw new Error(`Request failed with status ${res.status}: ${await res.text()}`);
    }
    async post(url, data, headers) {
        const res = await fetch(url, {
            headers: {
                ...headers,
                'Content-Type': 'application/json'
            },
            method: 'POST',
            body: JSON.stringify(data)
        });
        if (res.status === 401) {
            throw new AuthError();
        }
        if (res.ok) {
            return res.json();
        }
        throw new Error(`Request failed with status ${res.status}: ${await res.text()}`);
    }
}
//# sourceMappingURL=fetch-provider.connector.js.map