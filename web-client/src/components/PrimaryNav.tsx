import { h } from 'preact';
import { Link } from 'preact-router/match';
import CollapsibleList from '~components/CollapsibleList';
import { route } from 'preact-router';

interface Section {
    name: string;
    route: string;
    mobileOnly?: boolean;
}

const SECTIONS: Array<Section> = [
    {
        name: 'dashboard',
        route: '/',
        mobileOnly: true,
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
        name: 'media',
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
                    <Link onClick={(ev): void => ev.preventDefault()} activeClassName="active" href={section.route}>
                        {section.name}
                    </Link>
                );
            }}
            renderSelectorItem={(section: Section): string => section.name}
            onSelectItem={(section: Section): void => {
                route(section.route);
            }}
        />
    );
}
