export interface HotKeyCommand {
    ctrl?: boolean;
    alt?: boolean;
    shift?: boolean;
    meta?: boolean;
    key: string | Array<string>;
}

export function hotKeyCommandToString(cmd: HotKeyCommand): string {
    return `${cmd.shift ? '[shift]-' : ''}${cmd.ctrl ? '[ctrl]-' : ''}${cmd.alt ? '[alt]-' : ''}${
        cmd.meta ? '[meta]-' : ''
    }${Array.isArray(cmd.key) ? cmd.key.join('-') : cmd.key}`;
}
