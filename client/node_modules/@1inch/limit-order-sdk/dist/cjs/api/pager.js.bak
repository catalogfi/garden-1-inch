"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Pager = void 0;
const tslib_1 = require("tslib");
const assert_1 = tslib_1.__importDefault(require("assert"));
const validations_1 = require("../validations");
class Pager {
    constructor({ limit, page } = { page: 1, limit: 100 }) {
        (0, assert_1.default)((0, validations_1.isInt)(limit) && limit > 0, 'Invalid limit');
        (0, assert_1.default)((0, validations_1.isInt)(page) && page > 0, 'Invalid page');
        this.limit = limit;
        this.page = page;
    }
}
exports.Pager = Pager;
//# sourceMappingURL=pager.js.map