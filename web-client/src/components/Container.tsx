import { h } from 'preact';
import { useContext } from 'preact/hooks';

import { SITE_NAME } from '~config';
import Auth from '~auth/AuthContext';

import Sidebar from './Sidebar';

interface Props {
    children: h.JSX.Element | Array<h.JSX.Element>;
}

function Container(props: Props): h.JSX.Element {
    const authContext = useContext(Auth);

    return (
        <div>
            <nav className="navbar navbar-dark fixed-top bg-dark flex-md-nowrap p-0 shadow">
                <a className="navbar-brand col-sm-3 col-md-2 mr-0" href="/">
                    {SITE_NAME}
                </a>
                <input
                    className="form-control form-control-dark w-100"
                    type="text"
                    placeholder="search"
                    aria-label="search"
                />
                <ul className="navbar-nav px-3">
                    <li className="nav-item text-nowrap">
                        <a
                            className="nav-link"
                            href="#"
                            onClick={(): void => {
                                authContext.handleSignOut();
                            }}
                        >
                            sign out
                        </a>
                    </li>
                </ul>
            </nav>

            <div className="container-fluid">
                <div className="row">
                    <Sidebar />

                    <main role="main" className="col-md-9 ml-sm-auto col-lg-10 px-4">
                        {props.children}
                    </main>
                </div>
            </div>
        </div>
    );
}

export default Container;
