// @ts-ignore
import assert = require('assert');

export function assertCondition(condition: boolean, message?: string | Error): asserts condition {
    assert(condition, message);
}
