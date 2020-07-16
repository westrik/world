import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import AppContainer from '~components/AppContainer';
import CodeEditor, { EditorLanguage } from '~components/CodeEditor';
import LoadingSpinner from '~components/LoadingSpinner';
import { ApiNote, Content } from '~models/Note';
import fetchNote from '~notes/fetchNote';
import renderedContentToMarkdown from '~notes/renderedContentToMarkdown';
import Stack, { StackOrientation } from '~components/layout/Stack';
import NoteContent from '~notes/NoteContent';

interface Props {
    strippedApiId?: string;
}

export default function NoteEditor(props: Props): h.JSX.Element {
    const [rawContent, setRawContent] = useState<string | null>(null);
    const [renderedContent, setRenderedContent] = useState<Content | null>(null);
    const authContext = useContext(Auth);

    useEffect(() => {
        if (!renderedContent && props.strippedApiId) {
            fetchNote(authContext, `note_${props.strippedApiId}`, (note: ApiNote) => {
                if (note.content) {
                    setRenderedContent(note.content);
                    setRawContent(renderedContentToMarkdown(note.content));
                } else {
                    // TODO: error toast
                    setRenderedContent(null);
                    setRawContent(null);
                }
            });
        } else if (!props.strippedApiId) {
            setRenderedContent({
                elements: [],
                schemaVersion: 'v0.1.x',
            });
        }
    }, [authContext, rawContent, renderedContent, props.strippedApiId]);

    return (
        <AppContainer contentClassName="split-note-editor">
            {rawContent && renderedContent ? (
                <Stack orientation={StackOrientation.HORIZONTAL}>
                    <CodeEditor language={EditorLanguage.MARKDOWN} content={rawContent} />
                    <div className="note-preview">
                        <NoteContent elements={renderedContent.elements} />
                    </div>
                </Stack>
            ) : (
                <LoadingSpinner />
            )}
        </AppContainer>
    );
}
