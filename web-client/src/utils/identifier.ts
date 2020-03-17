export function stripApiId(apiId: string): string {
    return apiId.slice(apiId.indexOf('_') + 1);
}
