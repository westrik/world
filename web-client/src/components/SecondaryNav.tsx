import { h } from 'preact';
import { useContext } from 'preact/hooks';

import Auth from '~auth/AuthContext';

export default function SecondaryNav(): h.JSX.Element {
    const authContext = useContext(Auth);

    return (
        <ul className="nav-action-list">
            <li>
                <a
                    href="#"
                    onClick={(): void => {
                        authContext.handleSignOut();
                    }}
                >
                    sign out
                </a>
            </li>
        </ul>
    );
}
