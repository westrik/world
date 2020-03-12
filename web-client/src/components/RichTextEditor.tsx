import '../style/base.scss';

import { h } from 'preact';
import { useState } from 'preact/hooks';
import { NOTE_FIXTURE } from '~/../tests/fixtures/Notes';
import { Content, Element } from '~/models/Note';
import { ContentElement } from '~/notes/ContentElement';

export default function RichTextEditor(): h.JSX.Element {
    const [content, setContent] = useState<Content>(NOTE_FIXTURE.content!);
    return <ContentArea content={content} />;
}

interface ContentAreaProps {
    content: Content;
}
export function ContentArea(props: ContentAreaProps): h.JSX.Element {
    return (
        <div>
            {props.content.elements.map((el: Element, key: number) => (
                <ContentElement key={key} element={el} />
            ))}
        </div>
    );
}
