import { strict as assert } from 'assert';

export function assertCondition(condition: boolean, message?: string | Error): asserts condition {
    assert(condition, message);
}

export function fail(message: string): never {
    throw new Error(message);
}
