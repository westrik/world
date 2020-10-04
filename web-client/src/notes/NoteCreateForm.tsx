import { h } from 'preact';
import { TextField } from '~components/InputFields';
import { useState } from 'preact/hooks';

import SubmitButton from '~components/SubmitButton';
import { Note } from '~models/Note';

interface NoteCreateFormProps {
    onCreateNote: (note: Note) => void;
}

export default function NoteCreateForm(props: NoteCreateFormProps): h.JSX.Element {
    const [title, setTitle] = useState<string>('');

    return (
        <div>
            <TextField
                labelText="Title for new note"
                onChange={(event) => {
                    setTitle((event.target as HTMLInputElement).value);
                }}
            />
            {/* eslint-disable-next-line @typescript-eslint/no-empty-function */}
            <SubmitButton text="Create note" onButtonPress={() => {}} />
        </div>
    );
}
