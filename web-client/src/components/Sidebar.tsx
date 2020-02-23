import { h } from 'preact';
import { Link } from 'preact-router/match';

export default function Sidebar(): h.JSX.Element {
    // TODO: show nav somehow in mobile viewports
    return (
        <nav className="col-md-2 d-none d-md-block xs-block bg-light sidebar">
            <div className="sidebar-sticky">
                <ul className="nav flex-column">
                    <li className="nav-item">
                        <Link className="nav-link" activeClassName="active" href="/">
                            log
                        </Link>
                    </li>
                    <li className="nav-item">
                        <Link className="nav-link" activeClassName="active" href="/tasks">
                            tasks
                        </Link>
                    </li>
                    <li className="nav-item">
                        <Link class="nav-link" activeClassName="active" href="/docs">
                            documents
                        </Link>
                    </li>
                </ul>

                {/*TODO: disable admin section appropriately*/}
                <h6 className="sidebar-heading d-flex justify-content-between align-items-center px-3 mt-4 mb-1 text-muted">
                    <span>Admin</span>
                    <a className="d-flex align-items-center text-muted" href="#" aria-label="Admin" />
                </h6>
                <ul className="nav flex-column mb-2">
                    <li className="nav-item">
                        <Link class="nav-link" activeClassName="active" href="/users">
                            users
                        </Link>
                    </li>
                </ul>
            </div>
        </nav>
    );
}
