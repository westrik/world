import '../style/base.scss';

import { h } from 'preact';
import { useState } from 'preact/hooks';
import { NOTE_FIXTURE } from '~/../tests/fixtures/Notes';
import { Content, Element } from '~/models/Note';
import ContentElement from '~/notes/ContentElement';
import Container from '~components/Container';
import Header from '~components/Header';

interface Props {
    apiId?: string;
}

export default function NoteEditor(props: Props): h.JSX.Element {
    const [content, setContent] = useState<Content>(NOTE_FIXTURE.content!);
    // TODO: edit mode context
    return (
        <Container>
            <Header
                title={`editing ${props.apiId ? props.apiId.slice(5) : 'new document'}`}
                fixed={true}
                backLink="/notes"
                backLinkTitle="notes"
            />
            <div className="textEditor">
                {content.elements.map((el: Element, key: number) => (
                    <ContentElement key={key} element={el} />
                ))}
            </div>
        </Container>
    );
}
