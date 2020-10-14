import { expect } from 'chai';
import { HotKeyCommand, hotKeyCommandToString } from '~keyboard/HotKeyCommand';

describe('HotKeyContext', () => {
    it('converts hotkeys to strings', () => {
        let cmd: HotKeyCommand = {
            meta: true,
            alt: true,
            ctrl: true,
            key: ['a', 'b', 'c'],
        };
        expect(hotKeyCommandToString(cmd)).equal('[ctrl]-[alt]-[meta]-a-b-c');
        cmd = {
            shift: true,
            alt: true,
            ctrl: true,
            key: 'a',
        };
        expect(hotKeyCommandToString(cmd)).equal('[shift]-[ctrl]-[alt]-a');
    });
});
