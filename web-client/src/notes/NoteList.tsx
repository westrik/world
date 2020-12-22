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

enum RequestStateType {
    LOADING = 'loading',
    ERROR = 'error',
    COMPLETED = 'completed',
}

interface LoadingState {
    state: RequestStateType.LOADING;
    // TODO: number of attempts (when retries are implemented)
}

interface ErrorState {
    state: RequestStateType.ERROR;
    errorMessage: string;
}

interface CompletedState {
    state: RequestStateType.COMPLETED;
}

type RequestState = LoadingState | ErrorState | CompletedState;

export default function NoteList(): h.JSX.Element {
    const [requestState, setRequestState] = useState<RequestState>({ state: RequestStateType.LOADING });
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
            listNotes({
                authContext,
                handleReceiveResponse: (notes) => {
                    setRequestState({ state: RequestStateType.COMPLETED });
                    setNotes(notes ?? []);
                },
            }).catch(() => {
                setNotes([]);
                setRequestState({ state: RequestStateType.ERROR, errorMessage: 'Failed to load notes' });
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
            {requestState.state === RequestStateType.COMPLETED ? (
                <ListContainer className="notes">
                    {noteSummaries!.map((note, key) => (
                        <li className="note-item" key={key}>
                            <a href={`/notes/${stripPrefixFromId(note.id)}`}>{note.name}</a>
                        </li>
                    ))}
                </ListContainer>
            ) : requestState.state === RequestStateType.LOADING ? (
                <LoadingSpinner />
            ) : (
                // TODO: add button to retry failed request(s)
                <span className="error-message">{requestState.errorMessage}</span>
            )}
        </AppContainer>
    );
}
