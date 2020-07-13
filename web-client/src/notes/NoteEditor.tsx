import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import AppContainer from '~components/AppContainer';
import LoadingSpinner from '~components/LoadingSpinner';
import useMutationObserver from '~hooks/useMutationObserver';
import { ApiNote, Content, Element } from '~models/Note';
import fetchNote from '~notes/fetchNote';
import ContentElement from '~notes/ContentElement';
import { randomIdentifier } from '~utils/identifier';

interface Props {
    strippedApiId?: string;
}

export default function NoteEditor(props: Props): h.JSX.Element {
    const [content, setContent] = useState<Content | null>(null);
    const authContext = useContext(Auth);
    const editorId = `editor-${randomIdentifier()}`;

    useMutationObserver(true, editorId, (mutations: Array<MutationRecord>) => {
        // TODO: apply mutations to `content`
        // TODO: then call `setContent`
        // TODO: then call debounced DAO helper to sync w/ server

        console.log(`got ${mutations.length} mutations`);
        for (const mutation of mutations) {
            // TODO: fix type for this object
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            const mutationData: any = {
                type: mutation.type,
                oldValue: mutation.oldValue,
            };
            if (mutation.addedNodes) {
                mutationData['num_added'] = mutation.addedNodes.length;
                mutationData['added'] = mutation.addedNodes;
            }
            if (mutation.removedNodes) {
                mutationData['num_removed'] = mutation.removedNodes.length;
                mutationData['removed'] = mutation.removedNodes;
            }
            console.table(mutationData);
        }
    });

    useEffect(() => {
        if (!content && props.strippedApiId) {
            fetchNote(authContext, `note_${props.strippedApiId}`, (note: ApiNote) => {
                if (note.content) {
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
    }, [authContext, content, props.strippedApiId]);

    return (
        <AppContainer>
            <div className="textEditor">
                {content ? (
                    <div id={editorId} className="elements">
                        {content.elements.map((el: Element, key: number) => (
                            <ContentElement key={key} element={el} />
                        ))}
                    </div>
                ) : (
                    <LoadingSpinner />
                )}
            </div>
        </AppContainer>
    );
}
