import {
    ColumnType,
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
    TableData,
} from '~/models/Note';
import { h } from 'preact';
import { useContext } from 'preact/hooks';
import Editing from '~notes/EditingContext';

/*
TODO:
    when editing an element, set `contenteditable="true"` with:
        outline: none;
        border: 0;
 */

function renderElements(cxn: Array<Element> | null): Array<h.JSX.Element> | null {
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
    return <pre>{renderElements(props.cxn)}</pre>;
}

interface HeaderProps extends ElementWithChildrenProps {
    headerType: HeaderType;
}
function Header(props: HeaderProps): h.JSX.Element {
    const renderedChildren = renderElements(props.cxn);
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
            {renderElements(props.cxn)}
        </a>
    );
}

function Image(props: LinkProps): h.JSX.Element {
    const { destinationUrl, title } = props.link;
    return (
        <img src={destinationUrl} title={title} alt={title}>
            {renderElements(props.cxn)}
        </img>
    );
}

interface TableHeaderProps {
    element: Element;
    columnTypes: Array<ColumnType>;
}

function TableHeader(props: TableHeaderProps): h.JSX.Element {
    if (!props.element.children) {
        return <thead />;
    }
    return (
        <thead>
            {props.element.children.map((tableCell, key) => {
                if (isTableCell(tableCell.element)) {
                    return (
                        <th key={key} style={`text-align: ${props.columnTypes[key]}`}>
                            {renderElements(tableCell.children)}
                        </th>
                    );
                } else {
                    console.error('expected child of thead to be th');
                }
            })}
        </thead>
    );
}

interface TableRowProps {
    element: Element;
    columnTypes: Array<ColumnType>;
}

function TableRow(props: TableRowProps): h.JSX.Element {
    if (!props.element.children) {
        return <tr />;
    } else {
        return (
            <tr>
                {props.element.children.map((tableCell, key) => {
                    if (isTableCell(tableCell.element)) {
                        return (
                            <td key={key} style={`text-align: ${props.columnTypes[key]}}`}>
                                {renderElements(tableCell.children)}
                            </td>
                        );
                    } else {
                        console.error('expected child of tr to be td');
                    }
                })}
            </tr>
        );
    }
}

interface TableProps extends ElementWithChildrenProps {
    data: TableData;
}

function Table(props: TableProps): h.JSX.Element {
    if (!props.cxn) {
        return <table />;
    }
    const tableHeaders = props.cxn.filter(el => isTableHead(el.element));
    const tableRows = props.cxn.filter(el => isTableRow(el.element));
    return (
        <table>
            {tableHeaders.length > 0 ? (
                <TableHeader element={tableHeaders[0]} columnTypes={props.data.columnTypes} />
            ) : null}
            {tableRows.map((tableRow, key) => (
                <TableRow key={key} element={tableRow} columnTypes={props.data.columnTypes} />
            ))}
        </table>
    );
}

interface ContentElementProps {
    element: Element;
}

export default function ContentElement(props: ContentElementProps): h.JSX.Element {
    const { element, children } = props.element;
    const editingContext = useContext(Editing);

    if (isText(element)) {
        return <span contentEditable={editingContext.isEditing}>{element.text}</span>;
    } else if (isCode(element)) {
        return <code contentEditable={editingContext.isEditing}>{element.code}</code>;
    } else if (isHtml(element)) {
        // TODO
    } else if (isEmphasis(element)) {
        return <em contentEditable={editingContext.isEditing}>{renderElements(children)}</em>;
    } else if (isParagraph(element)) {
        return <p contentEditable={editingContext.isEditing}>{renderElements(children)}</p>;
    } else if (isStrong(element)) {
        return <strong contentEditable={editingContext.isEditing}>{renderElements(children)}</strong>;
    } else if (isStrikethrough(element)) {
        return <del contentEditable={editingContext.isEditing}>{renderElements(children)}</del>;
    } else if (isCodeBlock(element)) {
        return <CodeBlock language={element.codeBlock.language} cxn={children} />;
    } else if (isHeaderElement(element)) {
        return <Header headerType={element.header} cxn={children} />;
    } else if (isLink(element)) {
        return <Link link={element.link} cxn={children} />;
    } else if (isImage(element)) {
        return <Image link={element.image} cxn={children} />;
    } else if (isList(element)) {
        return <ul contentEditable={editingContext.isEditing}>{renderElements(children)}</ul>;
    } else if (isListItem(element)) {
        return <li contentEditable={editingContext.isEditing}>{renderElements(children)}</li>;
    } else if (isTaskListMarker(element)) {
        return <span contentEditable={editingContext.isEditing}>[{element.taskListMarker.checked ? 'x' : ' '}] </span>;
    } else if (isFootnoteDefinition(element)) {
        // TODO
    } else if (isFootnoteReference(element)) {
        return (
            <sup className="footnote-reference">
                <a href={`#${element.footnoteReference}`}>{element.footnoteReference}</a>
            </sup>
        );
    } else if (isTable(element)) {
        return <Table data={element.table} cxn={children} />;
    } else if (isSoftBreak(element)) {
        // TODO?
    } else if (isHardBreak(element)) {
        return <br />;
    } else if (isRule(element)) {
        return <hr />;
    }
    return <span />;
}
