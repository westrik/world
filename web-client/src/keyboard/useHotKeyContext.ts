import { createContext } from 'preact';
import { useContext, useEffect } from 'preact/hooks';
import { Code, HotKeyCommand, hotKeyCommandToString } from '~keyboard/HotKeyCommand';

export interface HotKeyContext {
    commandsToHandlers: { [command: string]: () => void };
    isDisabled?: boolean; // TODO: set when a vim-enabled editor is focused
}

type HotKeyToHandler = Map<HotKeyCommand, () => void>;

function registerHotKey(ctx: HotKeyContext, cmd: HotKeyCommand, handler: () => void): () => void {
    // TODO: validate that the cmd isn't already configured
    const command = hotKeyCommandToString(cmd);
    ctx.commandsToHandlers[command] = handler;
    return () => {
        delete ctx.commandsToHandlers[command];
    };
}

function createKeyDownHandler(ctx: HotKeyContext): (event: KeyboardEvent) => void {
    return (event: KeyboardEvent) => {
        if (ctx.isDisabled) {
            return;
        }
        const handler =
            ctx.commandsToHandlers[
                hotKeyCommandToString({
                    code: event.code as Code,
                    alt: event.altKey,
                    ctrl: event.ctrlKey,
                    meta: event.metaKey,
                    shift: event.shiftKey,
                })
            ];
        if (handler) {
            event.stopPropagation();
            handler();
        }
    };
}

const HotKey = createContext<HotKeyContext>({ commandsToHandlers: {} });

export default function useHotKeyContext(hotkeysToHandlers: HotKeyToHandler): void {
    const context = useContext(HotKey);
    useEffect(() => {
        const handleKeyDown = createKeyDownHandler(context);
        const unsubscribeCallbacks: Array<() => void> = [];
        hotkeysToHandlers.forEach((handler: () => void, cmd: HotKeyCommand) => {
            unsubscribeCallbacks.push(registerHotKey(context, cmd, handler));
        });
        window.addEventListener('keydown', handleKeyDown);
        return () => {
            window.removeEventListener('keydown', handleKeyDown);
            unsubscribeCallbacks.map((unsubscribe) => {
                unsubscribe();
            });
        };
    }, [context, hotkeysToHandlers]);
}

/** Navigation

 - `⌘` `K`  to open command menu
 then:

 - `d` - go to dashboard
 - `t` - go to tasks
 - `n` - go to notes listing
 - `m` - go to media listing
 - `s` - go to settings
 - continue typing to choose another command

 --------

 From the notes listing:

 - `c` - create a note

 -------

 From a note editor or misc text field:

 - `⌘` `Enter` - submit / save

 -------

 From the tasks listing:

 - `c` - create a task at the top of the list
 - `C` - create a task at the bottom of the list

*******/
