import { createContext } from 'preact';
import { useContext, useEffect } from 'preact/hooks';
import { HotKeyCommand } from '~keyboard/HotKeyCommand';

export interface HotKeyContext {
    hotkeysToHandlers: HotKeyToHandler;
    isDisabled?: boolean; // TODO: set when a vim-enabled editor is focused
}

type HotKeyToHandler = Map<HotKeyCommand, () => void>;

function registerHotKey(cxt: HotKeyContext, cmd: HotKeyCommand, handler: () => void): () => void {
    // TODO: validate that the cmd isn't already configured
    cxt.hotkeysToHandlers.set(cmd, handler);
    // TODO: register handler
    return () => {
        // TODO: deregister handler
        cxt.hotkeysToHandlers.delete(cmd);
    };
}

const HotKey = createContext<HotKeyContext>({ hotkeysToHandlers: new Map() });

export default function useHotKeyContext(hotkeysToHandlers: HotKeyToHandler): void {
    const context = useContext(HotKey);
    useEffect(() => {
        const unsubscribeCallbacks: Array<() => void> = [];
        hotkeysToHandlers.forEach((handler: () => void, cmd: HotKeyCommand) => {
            unsubscribeCallbacks.push(registerHotKey(context, cmd, handler));
        });
        // TODO: attach keyboard listener here?
        return () => {
            // TODO: detach keyboard listener here?
            unsubscribeCallbacks.map((callback) => {
                callback();
            });
        };
    }, [context, hotkeysToHandlers]);
}

/*******
 - `⌘` `K` - command menu
 - `⌘` `Enter` - submit / save

 --------

 ## Navigation

 - `⌘` `K`  to open command menu
 then:

 - `d` - go to dashboard
 - `t` - go to tasks
 - `n` - go to notes listing
 - `l` - go to library listing
 - `s` - go to settings
 - continue typing to choose another command

 --------

 From the notes listing:

 - `c` - create a note

 -------

 From the tasks listing:

 - `c` - create a task at the top of the list
 - `C` - create a task at the bottom of the list
*******/
