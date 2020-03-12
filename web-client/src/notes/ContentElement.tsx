import {
    Element,
    isCode,
    isCodeBlock,
    isEmphasis,
    isFootnoteDefinition,
    isFootnoteReference,
    isHardBreak,
    isHeaderElement,
    isHtml,
    isImage,
    isLink,
    isList,
    isListItem,
    isParagraph,
    isRule,
    isSoftBreak,
    isStrikethrough,
    isStrong,
    isTable,
    isTableCell,
    isTableHead,
    isTableRow,
    isTaskListMarker,
    isText,
} from '~/models/Note';
import { h } from 'preact';

interface ContentElementProps {
    element: Element;
}
export default function ContentElement(props: ContentElementProps): h.JSX.Element {
    const { element, children } = props.element;

    if (isText(element)) {
        return <span>{element.text}</span>;
    } else if (isCode(element)) {
        return <code>{element.code}</code>;
    } else if (isEmphasis(element) && children) {
        return (
            <em>
                {children.map((el, key) => (
                    <ContentElement element={el} key={key} />
                ))}
            </em>
        );
    } else if (isParagraph(element)) {
        return <Paragraph cxn={children} />;
    } else if (isCodeBlock(element)) {
        return <CodeBlock language={element.codeBlock.language} cxn={children} />;
    } else if (
        isHtml(element) ||
        isStrong(element) ||
        isStrikethrough(element) ||
        isHeaderElement(element) ||
        isLink(element) ||
        isImage(element) ||
        isList(element) ||
        isListItem(element) ||
        isTaskListMarker(element) ||
        isFootnoteDefinition(element) ||
        isFootnoteReference(element) ||
        isTable(element) ||
        isTableHead(element) ||
        isTableRow(element) ||
        isTableCell(element) ||
        isSoftBreak(element) ||
        isHardBreak(element) ||
        isRule(element)
    ) {
        return <span>unimplemented</span>;
    }
    return <span />;
}

interface ElementWithChildrenProps {
    cxn: Array<Element> | null;
}

function Paragraph(props: ElementWithChildrenProps): h.JSX.Element {
    return <p>{props.cxn ? props.cxn.map((el, key) => <ContentElement element={el} key={key} />) : null}</p>;
}

interface CodeBlockProps extends ElementWithChildrenProps {
    language: string | null;
}
function CodeBlock(props: CodeBlockProps): h.JSX.Element {
    return <pre>{props.cxn ? props.cxn.map((el, key) => <ContentElement element={el} key={key} />) : null}</pre>;
}
