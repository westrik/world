import '../style/base.scss';

import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';
import { ApiNote, Content, Element } from '~/models/Note';
import ContentElement from '~/notes/ContentElement';
import Container from '~components/Container';
import Header from '~components/Header';
import Auth from '~auth/AuthContext';
import LoadingSpinner from '~components/LoadingSpinner';
import fetchNote from '~notes/fetchNote';
import Editing from '~notes/EditingContext';

interface Props {
    strippedApiId?: string;
}

export default function NoteEditor(props: Props): h.JSX.Element {
    const [content, setContent] = useState<Content | null>(null);
    const authContext = useContext(Auth);
    const editingContext = useContext(Editing);

    useEffect(() => {
        if (!content && props.strippedApiId) {
            fetchNote(authContext, `note_${props.strippedApiId}`, (note: ApiNote) => {
                if (note.content) {
                    if (!editingContext.isEditing) {
                        editingContext.toggleEditing();
                    }
                    setContent(note.content);
                } else {
                    // TODO: error toast
                    setContent(null);
                }
            });
        } else if (!props.strippedApiId) {
            setContent({
                elements: [],
                schemaVersion: 'v0.1.x',
            });
        }
    });

    return (
        <Container>
            <Header
                title={`${props.strippedApiId ? props.strippedApiId : 'new document'}`}
                fixed={true}
                backLink="/notes"
                backLinkTitle="notes"
            />
            <div className="textEditor">
                {content ? (
                    content.elements.map((el: Element, key: number) => <ContentElement key={key} element={el} />)
                ) : (
                    <LoadingSpinner />
                )}
            </div>
        </Container>
    );
}
