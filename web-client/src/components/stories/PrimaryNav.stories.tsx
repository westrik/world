import { h } from 'preact';
import Tile from '~components/Tile';
import PrimaryNav from '~components/PrimaryNav';

export default { title: 'Section Navigation' };

export function uncollapsed(): h.JSX.Element {
    return <PrimaryNav />;
}

export function collapsed(): h.JSX.Element {
    return <PrimaryNav />;
}
