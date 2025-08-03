export function concatQueryParams(params) {
    if (!params) {
        return '';
    }
    const keys = Object.keys(params);
    if (keys.length === 0) {
        return '';
    }
    return ('?' +
        keys
            .reduce((a, k) => {
            if (!params[k]) {
                return a;
            }
            const value = params[k];
            a.push(k +
                '=' +
                encodeURIComponent(Array.isArray(value) ? value.join(',') : value));
            return a;
        }, [])
            .join('&'));
}
//# sourceMappingURL=params.js.map