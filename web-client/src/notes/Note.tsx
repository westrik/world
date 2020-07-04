import { h } from 'preact';
import { EditingProvider } from '~notes/EditingContext';
import NoteEditor from '~notes/NoteEditor';

interface Props {
    strippedApiId?: string;
}

export default function Note(props: Props): h.JSX.Element {
    return (
        <EditingProvider>
            <NoteEditor strippedApiId={props.strippedApiId} />
        </EditingProvider>
    );
}
