export interface HotKeyCommand {
    ctrl?: boolean;
    alt?: boolean;
    super?: boolean;
    key: string | Array<string>;
}

export function hotKeyCommandToString(cmd: HotKeyCommand): string {
    return `${cmd.ctrl ? 'ctrl-' : ''}${cmd.alt ? 'alt-' : ''}${cmd.super ? 'super-' : ''}${
        Array.isArray(cmd.key) ? cmd.key.join('-') : cmd.key
    }`;
}
