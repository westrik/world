import { h } from 'preact';
import { Link } from 'preact-router/match';
import { useContext } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import { SITE_NAME } from '~config';
import { IconType } from '~components/Icon';

export interface Section {
    name: string;
    route: string;
    desktopOnly?: boolean;
    mobileOnly?: boolean;
    iconType: IconType;
}

export const SECTIONS: Array<Section> = [
    {
        name: 'Dashboard',
        route: '/',
        desktopOnly: true,
        iconType: IconType.HOME,
    },
    {
        name: 'Tasks',
        route: '/tasks',
        iconType: IconType.TASKS,
    },
    {
        name: 'Notes',
        route: '/notes',
        iconType: IconType.MARKDOWN,
    },
    // {
    //     name: 'Code',
    //     route: '/branches',
    //     iconType: IconType.TERMINAL,
    // },
    {
        name: 'Media',
        route: '/media',
        iconType: IconType.COLLECTION,
    },
    {
        name: 'Settings',
        route: '/settings',
        iconType: IconType.GEAR,
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
                            <button
                                onClick={(): void => {
                                    authContext.handleSignOut();
                                }}
                            >
                                sign out
                            </button>
                        </li>
                    </ul>
                </div>
            </nav>
        </header>
    );
}
