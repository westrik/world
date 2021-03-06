import { h } from 'preact';
import { useContext, useEffect, useMemo, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import AppContainer from '~components/AppContainer';
import CodeEditor, { EditorLanguage } from '~components/CodeEditor';
import LoadingSpinner from '~components/LoadingSpinner';
import { ApiNote, Content } from '~models/Note';
import fetchNote from '~notes/fetchNote';
import renderedContentToMarkdown from '~notes/renderedContentToMarkdown';
import Stack, { StackOrientation } from '~components/layout/Stack';
import NoteContent from '~notes/NoteContent';
import updateNote from '~notes/updateNote';

interface NoteEditorProps {
    content: Content;
    onChange: (content: string) => void;
    onTriggerRender: () => void;
}

function NoteEditor(props: NoteEditorProps): h.JSX.Element {
    const originalContent = useMemo(() => renderedContentToMarkdown(props.content), [props.content]);
    return (
        <Stack orientation={StackOrientation.HORIZONTAL}>
            <CodeEditor language={EditorLanguage.MARKDOWN} content={originalContent} onChange={props.onChange} />
            <div className="note-preview">
                <div className="scrolling-viewport">
                    <NoteContent elements={props.content.elements} />
                </div>
                <div className="button-bar">
                    <button onClick={props.onTriggerRender}>Render</button>
                </div>
            </div>
        </Stack>
    );
}

interface Props {
    strippedApiId?: string;
}

export default function FetchingNoteEditor(props: Props): h.JSX.Element {
    const [contentJson, setContentJson] = useState<Content | null>(null);
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const [contentSource, setContentSource] = useState<string | null>(null);
    const authContext = useContext(Auth);

    useEffect(() => {
        if (!contentJson && props.strippedApiId) {
            fetchNote(authContext, `note_${props.strippedApiId}`, (note: ApiNote) => {
                if (note.content) {
                    setContentJson(note.content);
                } else {
                    // TODO: error toast
                    setContentJson(null);
                }
            });
        } else if (!props.strippedApiId) {
            setContentJson({
                elements: [],
                schemaVersion: 'v0.1.x',
            });
        }
    }, [authContext, contentJson, props.strippedApiId]);

    return (
        <AppContainer contentClassName="split-note-editor">
            {contentJson ? (
                <NoteEditor
                    content={contentJson}
                    onChange={setContentSource}
                    onTriggerRender={(): void => {
                        console.log(contentSource);
                        if (contentSource) {
                            updateNote(authContext, `note_${props.strippedApiId}`, contentSource, (note: ApiNote) => {
                                if (note.content) {
                                    setContentJson(note.content);
                                } else {
                                    // TODO: error toast
                                }
                            });
                        }
                    }}
                />
            ) : (
                <LoadingSpinner />
            )}
        </AppContainer>
    );
}
