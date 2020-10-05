import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import AppContainer from '~components/AppContainer';
import LoadingSpinner from '~components/LoadingSpinner';
import ListContainer from '~components/layout/ListContainer';
import { Note } from '~models/Note';
import listNotes from '~notes/listNotes';
import NoteCreateForm from '~notes/NoteCreateForm';
import { stripPrefixFromId } from '~utils/identifier';

export default function NoteList(): h.JSX.Element {
    const [noteSummaries, setNotes] = useState<Array<Note> | null>(null);
    const authContext = useContext(Auth);

    // TODO: refactor into custom hook
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
            <NoteCreateForm
                onCreateNote={(note: Note) => {
                    setNotes([note, ...(noteSummaries ?? [])]);
                }}
            />
            {noteSummaries ? (
                <ListContainer className="notes">
                    {noteSummaries.map((note, key) => (
                        <li draggable={true} className="note-item" key={key}>
                            <a href={`/notes/${stripPrefixFromId(note.id)}`}>{note.name}</a>
                        </li>
                    ))}
                </ListContainer>
            ) : (
                <LoadingSpinner />
            )}
        </AppContainer>
    );
}
