export interface HotKeyCommand {
    ctrl?: boolean;
    alt?: boolean;
    shift?: boolean;
    meta?: boolean;
    code: Code | Array<Code>;
}

export enum Code {
    ZERO = 'Digit0',
    ONE = 'Digit1',
    TWO = 'Digit2',
    THREE = 'Digit3',
    FOUR = 'Digit4',
    FIVE = 'Digit5',
    SIX = 'Digit6',
    SEVEN = 'Digit7',
    EIGHT = 'Digit8',
    NINE = 'Digit9',
    A = 'KeyA',
    B = 'KeyB',
    C = 'KeyC',
    D = 'KeyD',
    E = 'KeyE',
    F = 'KeyF',
    G = 'KeyG',
    H = 'KeyH',
    I = 'KeyI',
    J = 'KeyJ',
    K = 'KeyK',
    L = 'KeyL',
    M = 'KeyM',
    N = 'KeyN',
    O = 'KeyO',
    P = 'KeyP',
    Q = 'KeyQ',
    R = 'KeyR',
    S = 'KeyS',
    T = 'KeyT',
    U = 'KeyU',
    V = 'KeyV',
    W = 'KeyW',
    X = 'KeyX',
    Y = 'KeyY',
    Z = 'KeyZ',
}

function codeToLabel(keyCode: Code): string | null {
    return keyCode.replace('Key', '').replace('Digit', '').toLowerCase();
}

export function hotKeyCommandToString(cmd: HotKeyCommand): string {
    return `${cmd.shift ? '[shift]-' : ''}${cmd.ctrl ? '[ctrl]-' : ''}${cmd.alt ? '[alt]-' : ''}${
        cmd.meta ? '[meta]-' : ''
    }${Array.isArray(cmd.code) ? cmd.code.map(codeToLabel).join('-') : codeToLabel(cmd.code)}`;
}
