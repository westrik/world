import { h } from 'preact';

import NavBar from './NavBar';

interface AppContainerProps {
    children: h.JSX.Element | Array<h.JSX.Element>;
    contentClassName?: string;
}

export default function AppContainer(props: AppContainerProps): h.JSX.Element {
    return (
        <div className="app-container">
            <NavBar />

            <main className={props.contentClassName} role="main">
                {props.children}
            </main>
        </div>
    );
}
