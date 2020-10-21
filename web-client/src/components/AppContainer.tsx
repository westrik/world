import { h } from 'preact';

import useHotKeyContext from '~keyboard/useHotKeyContext';

import NavSidebar from './NavSidebar';

interface AppContainerProps {
    children: h.JSX.Element | Array<h.JSX.Element>;
    contentClassName?: string;
}

export default function AppContainer(props: AppContainerProps): h.JSX.Element {
    useHotKeyContext(
        new Map([
            [
                { meta: true, key: 'k' },
                () => {
                    console.log('hello from keyboard shortcut');
                },
            ],
        ]),
    );

    return (
        <div className="app-container">
            <NavSidebar />

            <main className={props.contentClassName} role="main">
                {props.children}
            </main>
        </div>
    );
}
