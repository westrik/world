import { h } from 'preact';
import { Link } from 'preact-router/match';

interface Section {
    name: string;
    route: string;
}

const SECTIONS: Array<Section> = [
    {
        name: 'dashboard',
        route: '/',
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
        route: '/',
    },
];

export default function PrimaryNav(): h.JSX.Element {
    return (
        <ul className="nav-section-list">
            {SECTIONS.map((section, key) => (
                <li>
                    <Link key={key} activeClassName="active" href={section.route}>
                        {section.name}
                    </Link>
                </li>
            ))}
        </ul>
    );
}
