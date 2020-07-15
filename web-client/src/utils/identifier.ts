export function stripPrefixFromId(id: string): string {
    return id.slice(id.indexOf('_') + 1);
}

export function randomIdentifier(): string {
    return Math.random().toString(36).substring(2, 15);
}
