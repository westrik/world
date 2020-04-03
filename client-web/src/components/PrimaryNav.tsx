import { h } from 'preact';
import { Link } from 'preact-router/match';
import CollapsibleList from '~components/CollapsibleList';
import { route } from 'preact-router';

interface Section {
    name: string;
    route: string;
    desktopOnly?: boolean;
    mobileOnly?: boolean;
}

const SECTIONS: Array<Section> = [
    {
        name: '-',
        route: '/',
        desktopOnly: true,
    },
    {
        name: 'Tasks',
        route: '/tasks',
    },
    {
        name: 'Notes',
        route: '/notes',
    },
    {
        name: 'Media',
        route: '/media',
    },
];

export default function PrimaryNav(): h.JSX.Element {
    const currentRoute = window.location.pathname;
    const sectionIndex = SECTIONS.findIndex(
        section => currentRoute == section.route || (currentRoute.includes(section.route) && !section.mobileOnly),
    );

    return (
        <CollapsibleList
            className="nav-section-list"
            items={SECTIONS}
            selectedIndex={sectionIndex}
            renderListItem={(section: Section): h.JSX.Element | null => {
                if (section.mobileOnly) {
                    return null;
                }
                return (
                    <Link
                        className="button inverted"
                        onClick={(ev): void => ev.preventDefault()}
                        activeClassName="active"
                        href={section.route}
                    >
                        {section.name}
                    </Link>
                );
            }}
            renderSelectorItem={(section: Section): string | null => {
                if (section.desktopOnly) {
                    return null;
                }
                return section.name;
            }}
            onSelectItem={(section: Section): void => {
                route(section.route);
            }}
        />
    );
}
