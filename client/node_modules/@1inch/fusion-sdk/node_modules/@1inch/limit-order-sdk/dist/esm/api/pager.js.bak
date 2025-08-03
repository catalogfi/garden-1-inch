import assert from 'assert';
import { isInt } from '../validations';
export class Pager {
    constructor({ limit, page } = { page: 1, limit: 100 }) {
        assert(isInt(limit) && limit > 0, 'Invalid limit');
        assert(isInt(page) && page > 0, 'Invalid page');
        this.limit = limit;
        this.page = page;
    }
}
//# sourceMappingURL=pager.js.map