import { h } from 'preact';

import NoteEditor from '~notes/NoteEditor';

interface Props {
    strippedApiId?: string;
}

// TODO: use code-splitting to load CodeMirror at runtime only
export default function Note(props: Props): h.JSX.Element {
    return <NoteEditor strippedApiId={props.strippedApiId} />;
}
