import { h } from 'preact';
import { Link } from 'preact-router/match';
import { route } from 'preact-router';
import { useContext } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import { SITE_NAME } from '~config';

interface Section {
    name: string;
    route: string;
    desktopOnly?: boolean;
    mobileOnly?: boolean;
}

const SECTIONS: Array<Section> = [
    {
        name: 'dashboard',
        route: '/',
        desktopOnly: true,
    },
    {
        name: 'tasks',
        route: '/tasks',
    },
    {
        name: 'notes',
        route: '/notes',
    },
    {
        name: 'library',
        route: '/library',
    },
    {
        name: 'branches',
        route: '/branches',
    },
];

function sectionClassName(section: Section): string | undefined {
    if (section.desktopOnly) {
        return 'desktop-only';
    } else if (section.mobileOnly) {
        return 'mobile-only';
    }
}

export default function NavBar(): h.JSX.Element {
    const authContext = useContext(Auth);
    return (
        <header className="nav-bar">
            <figure className="brand">
                <Link activeClassName="active" href="/">
                    {SITE_NAME}
                </Link>
            </figure>
            <nav className="menu">
                <input type="checkbox" id="menuToggle" />
                <label htmlFor="menuToggle" className="menu-icon">
                    MENU
                </label>
                <div>
                    <ul className="nav-section-list">
                        {SECTIONS.map((section, idx) => (
                            <li key={idx} className={sectionClassName(section)}>
                                <Link activeClassName="active" href={section.route}>
                                    {section.name}
                                </Link>
                            </li>
                        ))}
                    </ul>
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
                </div>
            </nav>
        </header>
    );
}
