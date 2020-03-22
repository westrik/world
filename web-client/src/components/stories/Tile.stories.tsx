import { h } from 'preact';
import Tile from '~components/Tile';

export default { title: 'Tile' };

export function withText(): h.JSX.Element {
    return (
        <Tile>
            <h2>Hello Tile</h2>
        </Tile>
    );
}
