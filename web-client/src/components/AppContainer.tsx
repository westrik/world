import { h } from 'preact';
import { route } from 'preact-router';

import useHotKeyContext from '~keyboard/useHotKeyContext';

import NavSidebar from './NavSidebar';
import { Code } from '~keyboard/HotKeyCommand';

interface AppContainerProps {
    sectionName?: string;
    contentClassName?: string;
    actionEls?: Array<h.JSX.Element>;
    children: h.JSX.Element | Array<h.JSX.Element | string | null>;
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

function SectionHeader(props: Pick<AppContainerProps, 'sectionName' | 'actionEls'>): h.JSX.Element {
    return (
        <div className="section-header">
            {props.sectionName ? <h2>{props.sectionName}</h2> : null}
            {props.actionEls ? (
                <ul className="actions-list">
                    {props.actionEls.map((el, key) => (
                        // TODO: if `el` is a Button, assert that size==SMALL
                        <li key={key}>{el}</li>
                    ))}
                </ul>
            ) : null}
        </div>
    );
}

export default function AppContainer(props: AppContainerProps): h.JSX.Element {
    useHotKeyContext(HOTKEYS);

    // TODO: context / provider for modals
    // TODO: context / provider for toasts + error management

    return (
        <div className="app-container">
            <NavSidebar />
            <main className={props.contentClassName} role="main">
                {props.sectionName || props.actionEls ? (
                    <SectionHeader sectionName={props.sectionName} actionEls={props.actionEls} />
                ) : null}
                {props.children}
            </main>
            {/* TODO: mount point for modals */}
            {/* TODO: mount point for toasts */}
        </div>
    );
}
