"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.AxiosProviderConnector = void 0;
const tslib_1 = require("tslib");
const axios_1 = tslib_1.__importStar(require("axios"));
const errors_1 = require("../../errors");
class AxiosProviderConnector {
    async get(url, headers) {
        try {
            const res = await axios_1.default.get(url, {
                headers
            });
            return res.data;
        }
        catch (error) {
            if ((0, axios_1.isAxiosError)(error) && error.response?.status === 401) {
                throw new errors_1.AuthError();
            }
            throw error;
        }
    }
    async post(url, data, headers) {
        try {
            const res = await axios_1.default.post(url, data, {
                headers
            });
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