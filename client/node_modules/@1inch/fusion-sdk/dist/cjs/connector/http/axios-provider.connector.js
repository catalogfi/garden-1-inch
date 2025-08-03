"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.AxiosProviderConnector = void 0;
const tslib_1 = require("tslib");
const axios_1 = tslib_1.__importStar(require("axios"));
const errors_1 = require("../../errors");
class AxiosProviderConnector {
    constructor(authKey) {
        this.authKey = authKey;
    }
    async get(url) {
        try {
            const res = await axios_1.default.get(url, this.authKey
                ? {
                    headers: {
                        Authorization: `Bearer ${this.authKey}`
                    }
                }
                : undefined);
            return res.data;
        }
        catch (error) {
            if ((0, axios_1.isAxiosError)(error) && error.response?.status === 401) {
                throw new errors_1.AuthError();
            }
            throw error;
        }
    }
    async post(url, data) {
        try {
            const res = await axios_1.default.post(url, data, this.authKey
                ? {
                    headers: {
                        Authorization: `Bearer ${this.authKey}`
                    }
                }
                : undefined);
            return res.data;
        }
        catch (error) {
            if ((0, axios_1.isAxiosError)(error) && error.response?.status === 401) {
                throw new errors_1.AuthError();
            }
            throw error;
        }
    }
}
exports.AxiosProviderConnector = AxiosProviderConnector;
//# sourceMappingURL=axios-provider.connector.js.map