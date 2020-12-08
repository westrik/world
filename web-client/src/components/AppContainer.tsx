import { h } from 'preact';
import { route } from 'preact-router';

import useHotKeyContext from '~keyboard/useHotKeyContext';

import NavSidebar from './NavSidebar';
import { Code } from '~keyboard/HotKeyCommand';

interface AppContainerProps {
    sectionName?: string;
    contentClassName?: string;
    children: h.JSX.Element | Array<h.JSX.Element>;
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
            route('/media');
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
                {props.sectionName ? <h2 className="section-header">{props.sectionName}</h2> : null}
                {props.children}
            </main>
        </div>
    );
}
