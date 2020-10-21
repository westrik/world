import { h } from 'preact';
import { Link } from 'preact-router/match';
import { useContext } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import { SITE_NAME } from '~config';
import { Section, SECTIONS } from '~components/NavBar';

function sectionClassName(section: Section): string | undefined {
    if (section.desktopOnly) {
        return 'desktop-only';
    } else if (section.mobileOnly) {
        return 'mobile-only';
    }
}

export default function NavSidebar(): h.JSX.Element {
    const authContext = useContext(Auth);
    return (
        <div className="nav-sidebar">
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
                                Sign Out
                            </button>
                        </li>
                    </ul>
                </div>
            </nav>
        </div>
    );
}
