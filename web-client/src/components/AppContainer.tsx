import { h } from 'preact';

import NavBar from './NavBar';

interface AppContainerProps {
    children: h.JSX.Element | Array<h.JSX.Element>;
}

export default function AppContainer(props: AppContainerProps): h.JSX.Element {
    return (
        <div className="app-container">
            <NavBar />

            <main role="main">{props.children}</main>
        </div>
    );
}
