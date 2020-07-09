import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import AppContainer from '~components/AppContainer';
import ListContainer from '~components/ListContainer';
import LoadingSpinner from '~components/LoadingSpinner';
import { Note } from '~models/Note';
import listNotes from '~notes/listNotes';

export default function NoteList(): h.JSX.Element {
    const [noteSummaries, setNotes] = useState<Array<Note> | null>(null);
    const authContext = useContext(Auth);

    useEffect(() => {
        if (!noteSummaries) {
            listNotes(authContext, (notes) => {
                if (notes) {
                    setNotes(notes);
                } else {
                    setNotes([]);
                }
            });
        }
    });

    return (
        <AppContainer>
            {noteSummaries ? (
                <ListContainer className="notes">
                    {noteSummaries.map((note, key) => (
                        <li draggable={true} className="note-item" key={key}>
                            {note.apiId}
                        </li>
                    ))}
                </ListContainer>
            ) : (
                <LoadingSpinner />
            )}
        </AppContainer>
    );
}
