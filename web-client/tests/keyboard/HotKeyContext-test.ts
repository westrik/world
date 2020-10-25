import { expect } from 'chai';
import { Code, HotKeyCommand, hotKeyCommandToString } from '~keyboard/HotKeyCommand';

describe('HotKeyContext', () => {
    it('converts hotkeys to strings', () => {
        let cmd: HotKeyCommand = {
            meta: true,
            alt: true,
            ctrl: true,
            code: [Code.A, Code.B, Code.C],
        };
        expect(hotKeyCommandToString(cmd)).equal('[ctrl]-[alt]-[meta]-a-b-c');
        cmd = {
            shift: true,
            alt: true,
            ctrl: true,
            code: Code.A,
        };
        expect(hotKeyCommandToString(cmd)).equal('[shift]-[ctrl]-[alt]-a');
    });
});
