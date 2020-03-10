import { h } from 'preact';

import Container from '~components/Container';
import Header from '~components/Header';
import { useContext, useEffect, useState } from 'preact/hooks';
import { Note } from '~models/Note';
import Auth from '~auth/AuthContext';
import listNotes from '~notes/listNotes';
import ListContainer from '~components/ListContainer';

function NoteList(): h.JSX.Element {
    const [notes, setNotes] = useState<Array<Note> | null>(null);
    const authContext = useContext(Auth);

    useEffect(() => {
        if (!notes) {
            listNotes(authContext, notes => {
                if (notes) {
                    setNotes(notes);
                } else {
                    setNotes([]);
                }
            });
        }
    });

    return (
        <Container>
            <Header title="notes" fixed={true} />
            {notes ? (
                <ListContainer>
                    {notes.map((note, key) => (
                        <div key={key}>{note.createdAt}</div>
                    ))}
                </ListContainer>
            ) : (
                <br />
            )}
        </Container>
    );
}

export default NoteList;
