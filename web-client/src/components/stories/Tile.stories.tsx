import { h } from 'preact';
import Tile from '~components/Tile';

export default { title: 'Tile' };

export function withText(): h.JSX.Element {
    return (
        <ul className="story-container">
            <li>
                <Tile>
                    <h1>Hello</h1>
                </Tile>
            </li>
            <li>
                <Tile>
                    <h2>tile</h2>
                </Tile>
            </li>
            <li>
                <Tile>
                    <h3>world</h3>
                </Tile>
            </li>
        </ul>
    );
}
