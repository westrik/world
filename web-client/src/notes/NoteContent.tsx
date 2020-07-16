import { h } from 'preact';

import { Element } from '~models/Note';
import ContentElement from '~notes/ContentElement';

interface Props {
    elements: Array<Element>;
}

export default function NoteContent(props: Props): h.JSX.Element {
    return (
        <div className="article">
            {props.elements.map((el: Element, key: number) => (
                <ContentElement key={key} element={el} />
            ))}
        </div>
    );
}
