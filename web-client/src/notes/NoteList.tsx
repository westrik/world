import { h } from 'preact';

import Container from '~components/Container';
import Header from '~components/Header';
import { useState } from 'preact/hooks';
import { Note } from '~models/Note';

function NoteList(): h.JSX.Element {
    const [notes, setNotes] = useState<Array<Note> | null>(null);

    return (
        <Container>
            <Header title="notes" fixed={true} />
        </Container>
    );
}

export default NoteList;
