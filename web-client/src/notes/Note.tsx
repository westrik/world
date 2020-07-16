import { h } from 'preact';

import NoteEditor from '~notes/NoteEditor';

interface Props {
    strippedApiId?: string;
}

// TODO: remove this whole component?
export default function Note(props: Props): h.JSX.Element {
    return <NoteEditor strippedApiId={props.strippedApiId} />;
}
