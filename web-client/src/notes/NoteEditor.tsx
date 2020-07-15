import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import AppContainer from '~components/AppContainer';
import CodeEditor, { EditorLanguage } from '~components/CodeEditor';
import LoadingSpinner from '~components/LoadingSpinner';
import { ApiNote, Content, Element } from '~models/Note';
import fetchNote from '~notes/fetchNote';
import ContentElement from '~notes/ContentElement';
import renderedContentToMarkdown from '~notes/renderedContentToMarkdown';

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
        <AppContainer>
            {rawContent ? <CodeEditor language={EditorLanguage.MARKDOWN} content={rawContent} /> : <LoadingSpinner />}
            {renderedContent ? (
                <div className="article elements">
                    {renderedContent.elements.map((el: Element, key: number) => (
                        <ContentElement key={key} element={el} />
                    ))}
                </div>
            ) : (
                <LoadingSpinner />
            )}
        </AppContainer>
    );
}
