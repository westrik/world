import {
    Element,
    HeaderType,
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
    LinkData,
} from '~/models/Note';
import { h } from 'preact';

function renderChildren(cxn: Array<Element> | null): Array<h.JSX.Element> | null {
    return cxn ? cxn.map((el, key) => <ContentElement element={el} key={key} />) : null;
}

interface ElementWithChildrenProps {
    cxn: Array<Element> | null;
}

interface CodeBlockProps extends ElementWithChildrenProps {
    language: string | null;
}
function CodeBlock(props: CodeBlockProps): h.JSX.Element {
    // TODO: language-specific code formatting
    return <pre>{renderChildren(props.cxn)}</pre>;
}

interface HeaderProps extends ElementWithChildrenProps {
    headerType: HeaderType;
}
function Header(props: HeaderProps): h.JSX.Element {
    const renderedChildren = renderChildren(props.cxn);
    if (props.headerType == HeaderType.H1) {
        return <h1>{renderedChildren}</h1>;
    } else if (props.headerType == HeaderType.H2) {
        return <h2>{renderedChildren}</h2>;
    } else if (props.headerType == HeaderType.H3) {
        return <h3>{renderedChildren}</h3>;
    } else if (props.headerType == HeaderType.H4) {
        return <h4>{renderedChildren}</h4>;
    } else if (props.headerType == HeaderType.H5) {
        return <h5>{renderedChildren}</h5>;
    } else {
        return <h6>{renderedChildren}</h6>;
    }
}

interface LinkProps extends ElementWithChildrenProps {
    link: LinkData;
}
function Link(props: LinkProps): h.JSX.Element {
    const { destinationUrl, title } = props.link;
    return (
        <a href={destinationUrl} title={title}>
            {renderChildren(props.cxn)}
        </a>
    );
}

function Image(props: LinkProps): h.JSX.Element {
    const { destinationUrl, title } = props.link;
    return (
        <img src={destinationUrl} title={title} alt={title}>
            {renderChildren(props.cxn)}
        </img>
    );
}

interface ContentElementProps {
    element: Element;
}

export default function ContentElement(props: ContentElementProps): h.JSX.Element {
    const { element, children } = props.element;

    if (isText(element)) {
        return <span>{element.text}</span>;
    } else if (isCode(element)) {
        return <code>{element.code}</code>;
    } else if (isHtml(element)) {
        // TODO
    } else if (isEmphasis(element)) {
        return <em>{renderChildren(children)}</em>;
    } else if (isParagraph(element)) {
        return <p>{renderChildren(children)}</p>;
    } else if (isStrong(element)) {
        return <strong>{renderChildren(children)}</strong>;
    } else if (isStrikethrough(element)) {
        return <del>{renderChildren(children)}</del>;
    } else if (isCodeBlock(element)) {
        return <CodeBlock language={element.codeBlock.language} cxn={children} />;
    } else if (isHeaderElement(element)) {
        return <Header headerType={element.header} cxn={children} />;
    } else if (isLink(element)) {
        return <Link link={element.link} cxn={children} />;
    } else if (isImage(element)) {
        return <Image link={element.image} cxn={children} />;
    } else if (isList(element)) {
        return <ul>{renderChildren(children)}</ul>;
    } else if (isListItem(element)) {
        return <li>{renderChildren(children)}</li>;
    } else if (isTaskListMarker(element)) {
        return <span>[{element.taskListMarker.checked}]</span>;
    } else if (isFootnoteDefinition(element)) {
        // TODO
    } else if (isFootnoteReference(element)) {
        // TODO
    } else if (isTable(element)) {
        // TODO
    } else if (isTableHead(element)) {
        // TODO
    } else if (isTableRow(element)) {
        // TODO
    } else if (isTableCell(element)) {
        // TODO
    } else if (isSoftBreak(element)) {
        // TODO: do nothing??
    } else if (isHardBreak(element)) {
        return <br />;
    } else if (isRule(element)) {
        return <hr />;
    }
    return <span />;
}
