import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import Container from '~components/Container';
import ListContainer from '~components/ListContainer';
import LoadingSpinner from '~components/LoadingSpinner';
import Header from '~components/Header';
import { Note } from '~models/Note';
import listNotes from '~notes/listNotes';
import { stripApiId } from '~utils/identifier';

function NoteList(): h.JSX.Element {
    const [noteSummaries, setNotes] = useState<Array<Note> | null>(null);
    const authContext = useContext(Auth);

    useEffect(() => {
        if (!noteSummaries) {
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
            {noteSummaries ? (
                <ListContainer>
                    {noteSummaries.map((note, key) => (
                        <li key={key}>
                            <a href={`/notes/${stripApiId(note.apiId)}`}>{stripApiId(note.apiId)}</a>
                        </li>
                    ))}
                </ListContainer>
            ) : (
                <LoadingSpinner />
            )}
        </Container>
    );
}

export default NoteList;
