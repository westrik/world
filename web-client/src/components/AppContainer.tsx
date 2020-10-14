import { h } from 'preact';

import NavBar from './NavBar';
import useHotKeyContext from '~keyboard/useHotKeyContext';

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
            <NavBar />

            <main className={props.contentClassName} role="main">
                {props.children}
            </main>
        </div>
    );
}
