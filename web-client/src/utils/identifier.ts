export function stripApiId(apiId: string): string {
    return apiId.slice(apiId.indexOf('_') + 1);
}

export function randomIdentifier(): string {
    return Math.random().toString(36).substring(2, 15);
}
