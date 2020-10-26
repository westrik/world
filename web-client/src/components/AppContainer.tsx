import { h } from 'preact';
import { route } from 'preact-router';

import useHotKeyContext from '~keyboard/useHotKeyContext';

import NavSidebar from './NavSidebar';
import { Code } from '~keyboard/HotKeyCommand';

interface AppContainerProps {
    children: h.JSX.Element | Array<h.JSX.Element>;
    contentClassName?: string;
}

const HOTKEYS = new Map([
    [
        { meta: true, code: Code.K },
        () => {
            console.log('TODO: trigger command menu');
        },
    ],
    [
        { alt: true, code: Code.ONE },
        () => {
            route('/');
        },
    ],
    [
        { alt: true, code: Code.TWO },
        () => {
            route('/tasks');
        },
    ],
    [
        { alt: true, code: Code.THREE },
        () => {
            route('/notes');
        },
    ],
    [
        { alt: true, code: Code.FOUR },
        () => {
            route('/library');
        },
    ],
    [
        { alt: true, code: Code.FIVE },
        () => {
            route('/settings');
        },
    ],
]);

export default function AppContainer(props: AppContainerProps): h.JSX.Element {
    useHotKeyContext(HOTKEYS);

    return (
        <div className="app-container">
            <NavSidebar />

            <main className={props.contentClassName} role="main">
                {props.children}
            </main>
        </div>
    );
}
