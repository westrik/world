import { h } from 'preact';

import { SITE_NAME } from '~config';

import PrimaryNav from './PrimaryNav';
import SecondaryNav from '~components/SecondaryNav';

interface Props {
    children: h.JSX.Element | Array<h.JSX.Element>;
}

function Container(props: Props): h.JSX.Element {
    return (
        <div className="app-container">
            <nav>
                <a href="/">{SITE_NAME}</a>
                <PrimaryNav />
                <SecondaryNav />
            </nav>

            <main role="main">{props.children}</main>
        </div>
    );
}

export default Container;
