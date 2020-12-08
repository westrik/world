import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import ListContainer from '~components/layout/ListContainer';
import AppContainer from '~components/AppContainer';
import LoadingSpinner from '~components/LoadingSpinner';
import useHotKeyContext from '~keyboard/useHotKeyContext';
import { Code } from '~keyboard/HotKeyCommand';
import { Note } from '~models/Note';
import NoteCreateForm from '~notes/NoteCreateForm';
import listNotes from '~notes/listNotes';
import { stripPrefixFromId } from '~utils/identifier';

export default function NoteList(): h.JSX.Element {
    const [noteSummaries, setNotes] = useState<Array<Note> | null>(null);
    const authContext = useContext(Auth);
    useHotKeyContext(
        new Map([
            [
                { code: Code.C },
                () => {
                    console.log('creating note');
                },
            ],
        ]),
    );

    // TODO: refactor into custom hook
    useEffect(() => {
        if (!noteSummaries) {
            listNotes(authContext, (notes) => {
                setNotes(notes ?? []);
            });
        }
    });

    return (
        <AppContainer sectionName="Notes">
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
