import { strict as assert } from 'assert';

export function assertCondition(condition: boolean, message?: string | Error): asserts condition {
    assert(condition, message);
}
