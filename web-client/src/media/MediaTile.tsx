import { stripApiId } from '~utils/identifier';
import { h } from 'preact';
import { Note } from '~models/Note';
import Tile from '~components/Tile';

interface Props {
    note: Note;
}

export default function MediaTile(props: Props): h.JSX.Element {
    return (
        <Tile>
            <a href={`/notes/${stripApiId(props.note.apiId)}`}>{stripApiId(props.note.apiId)}</a>
        </Tile>
    );
}
