import '../style/base.scss';

import { h } from 'preact';
import { useState } from 'preact/hooks';
import { NOTE_FIXTURE } from '~/../tests/fixtures/Notes';
import {
    CodeBlockElement,
    CodeElement,
    Content,
    Element,
    ElementData,
    EmphasisElement,
    FootnoteDefinitionElement,
    FootnoteReferenceElement,
    HardBreakElement,
    HeaderElement,
    HtmlElement,
    ImageElement,
    LinkElement,
    ListElement,
    ListItemElement,
    ParagraphElement,
    RuleElement,
    SoftBreakElement,
    StrikethroughElement,
    StrongElement,
    TableCellElement,
    TableElement,
    TableHeadElement,
    TableRowElement,
    TaskListMarkerElement,
    TextElement,
} from '~/models/Note';

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

function elementDataHasProperty(element: ElementData, property: string) {
    return Object.prototype.hasOwnProperty.call(element, property);
}

function isText(el: ElementData): el is TextElement {
    return elementDataHasProperty(el, 'text');
}
function isCode(el: ElementData): el is CodeElement {
    return elementDataHasProperty(el, 'code');
}
function isHtml(el: ElementData): el is HtmlElement {
    return elementDataHasProperty(el, 'html');
}
function isParagraph(el: ElementData): el is ParagraphElement {
    return el == 'p';
}
function isEmphasis(el: ElementData): el is EmphasisElement {
    return el == 'em';
}
function isStrong(el: ElementData): el is StrongElement {
    return el == 'strong';
}
function isStrikethrough(el: ElementData): el is StrikethroughElement {
    return el == 'strike';
}
function isHeaderElement(el: ElementData): el is HeaderElement {
    return elementDataHasProperty(el, 'header');
}
function isLink(el: ElementData): el is LinkElement {
    return elementDataHasProperty(el, 'link');
}
function isImage(el: ElementData): el is ImageElement {
    return elementDataHasProperty(el, 'image');
}
function isCodeBlock(el: ElementData): el is CodeBlockElement {
    return elementDataHasProperty(el, 'codeBlock');
}
function isList(el: ElementData): el is ListElement {
    return elementDataHasProperty(el, 'list');
}
function isListItem(el: ElementData): el is ListItemElement {
    return elementDataHasProperty(el, 'listItem');
}
function isTaskListMarker(el: ElementData): el is TaskListMarkerElement {
    return elementDataHasProperty(el, 'taskListMarker');
}
function isFootnoteDefinition(el: ElementData): el is FootnoteDefinitionElement {
    return elementDataHasProperty(el, 'footnoteDefinition');
}
function isFootnoteReference(el: ElementData): el is FootnoteReferenceElement {
    return elementDataHasProperty(el, 'footnoteReference');
}
function isTable(el: ElementData): el is TableElement {
    return elementDataHasProperty(el, 'table');
}
function isTableHead(el: ElementData): el is TableHeadElement {
    return el == 'tableHead';
}
function isTableRow(el: ElementData): el is TableRowElement {
    return el == 'tableRow';
}
function isTableCell(el: ElementData): el is TableCellElement {
    return el == 'tableCell';
}
function isSoftBreak(el: ElementData): el is SoftBreakElement {
    return el == 'softBreak';
}
function isHardBreak(el: ElementData): el is HardBreakElement {
    return el == 'hardBreak';
}
function isRule(el: ElementData): el is RuleElement {
    return el == 'rule';
}

interface ContentElementProps {
    element: Element;
}
function ContentElement(props: ContentElementProps): h.JSX.Element {
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
        isEmphasis(element) ||
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

interface ParagraphProps {
    cxn: Array<Element> | null;
}
function Paragraph(props: ParagraphProps): h.JSX.Element {
    return <p>{props.cxn ? props.cxn.map((el, key) => <ContentElement element={el} key={key} />) : null}</p>;
}

interface CodeBlockProps {
    language: string | null;
    cxn: Array<Element> | null;
}
function CodeBlock(props: CodeBlockProps): h.JSX.Element {
    return <pre>{props.cxn ? props.cxn.map((el, key) => <ContentElement element={el} key={key} />) : null}</pre>;
}
