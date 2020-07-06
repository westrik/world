import { h } from 'preact';

import NavBar from './NavBar';

interface Props {
    children: h.JSX.Element | Array<h.JSX.Element>;
}

function Container(props: Props): h.JSX.Element {
    return (
        <div className="app-container">
            <NavBar />

            <main role="main">{props.children}</main>
        </div>
    );
}

export default Container;
