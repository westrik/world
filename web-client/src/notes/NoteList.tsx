import { h } from 'preact';

import Container from '~components/Container';
import Header from '~components/Header';
import { useContext, useEffect, useState } from 'preact/hooks';
import { Note } from '~models/Note';
import Auth from '~auth/AuthContext';
import listNotes from '~notes/listNotes';
import ListContainer from '~components/ListContainer';
import LoadingSpinner from '~components/LoadingSpinner';

interface Props {
    apiId?: string;
}

function NoteList(props: Props): h.JSX.Element {
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
                            <a href={`/notes/${note.apiId}`}>{note.apiId.slice(5)}</a>
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
