import { h } from 'preact';
import { TextField } from '~components/InputFields';
import { useContext, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import SubmitButton from '~components/SubmitButton';
import { Note } from '~models/Note';
import createNote from '~notes/createNote';

interface NoteCreateFormProps {
    onCreateNote: (note: Note) => void;
}

export default function NoteCreateForm(props: NoteCreateFormProps): h.JSX.Element {
    const authContext = useContext(Auth);
    const [name, setName] = useState<string>('');

    return (
        <div>
            <TextField
                labelText="Name for new note"
                onChange={(event) => {
                    setName((event.target as HTMLInputElement).value);
                }}
            />
            <SubmitButton
                text="Create note"
                onButtonPress={async () => {
                    // eslint-disable-next-line @typescript-eslint/no-empty-function
                    await createNote(authContext, name, () => {});
                }}
            />
        </div>
    );
}
