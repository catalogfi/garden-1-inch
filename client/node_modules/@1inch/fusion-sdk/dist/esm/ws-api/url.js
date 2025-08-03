export function castUrl(url) {
    if (url.startsWith('http')) {
        return url.replace('http', 'ws');
    }
    return url;
}
//# sourceMappingURL=url.js.map