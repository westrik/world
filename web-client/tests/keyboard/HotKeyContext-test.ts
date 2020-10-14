import { expect } from 'chai';
import { HotKeyCommand, hotKeyCommandToString } from '~keyboard/HotKeyCommand';

describe('HotKeyContext', () => {
    it('converts hotkeys to strings', () => {
        let cmd: HotKeyCommand = {
            super: true,
            alt: true,
            ctrl: true,
            key: ['a', 'b', 'c'],
        };
        expect(hotKeyCommandToString(cmd)).equal('ctrl-alt-super-a-b-c');
        cmd = {
            alt: true,
            ctrl: true,
            key: 'a',
        };
        expect(hotKeyCommandToString(cmd)).equal('ctrl-alt-a');
    });
});
