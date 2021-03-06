import {
    ColumnType,
    Element,
    HeaderType,
    isBlockElement,
    isBlockQuote,
    isCode,
    isCodeBlock,
    isEmphasis,
    isFootnoteDefinition,
    isFootnoteReference,
    isHardBreak,
    isHeaderElement,
    isHtml,
    isImage,
    isInlineElement,
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
import { assertCondition } from '~utils/asserts';

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
    // eslint-disable-next-line react/no-unknown-property
    return <pre spellcheck={false}>{renderElements(props.cxn)}</pre>;
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

    {
        /* TODO: open internal links in new ww_tab */
    }
    return (
        <a href={destinationUrl} target="_blank" rel="noopener noreferrer" title={title}>
            {renderElements(props.cxn)}
        </a>
    );
}

function Image(props: LinkProps): h.JSX.Element {
    const { destinationUrl, title } = props.link;
    return (
        <div className="image-container">
            <img src={destinationUrl} title={title} alt={title}>
                {renderElements(props.cxn)}
            </img>
        </div>
    );
}

interface TaskListMarkerProps {
    checked: boolean;
}

function TaskListMarker(props: TaskListMarkerProps): h.JSX.Element {
    return <input type="checkbox" checked={props.checked} />;
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
    const tableHeaders = props.cxn.filter((el) => isTableHead(el.element));
    const tableRows = props.cxn.filter((el) => isTableRow(el.element));
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

interface BlockElementProps {
    element: Element;
}
function BlockElement(props: BlockElementProps): h.JSX.Element {
    const { element, children } = props.element;
    if (isHtml(element)) {
        // TODO: strip and validate on server-side; set with dangerouslySetInnerHTML
        return <div className="html-container" />;
    } else if (isParagraph(element)) {
        return <p>{renderElements(children)}</p>;
    } else if (isCodeBlock(element)) {
        return <CodeBlock language={element.codeBlock.language} cxn={children} />;
    } else if (isHeaderElement(element)) {
        return <Header headerType={element.header} cxn={children} />;
    } else if (isImage(element)) {
        return <Image link={element.image} cxn={children} />;
    } else if (isList(element)) {
        return <ul>{renderElements(children)}</ul>;
    } else if (isBlockQuote(element)) {
        return <blockquote>{renderElements(children)}</blockquote>;
    } else if (isFootnoteDefinition(element)) {
        // TODO
    } else if (isTable(element)) {
        return <Table data={element.table} cxn={children} />;
    } else if (isSoftBreak(element)) {
        return (
            <span>
                {' '}
                <wbr />
            </span>
        );
    } else if (isHardBreak(element)) {
        return <br />;
    } else if (isRule(element)) {
        return <hr />;
    }
    assertCondition(false, `Unsupported block element!: ${element}`);
}

interface InlineElementProps {
    element: Element;
}
function InlineElement(props: InlineElementProps): h.JSX.Element {
    const { element, children } = props.element;
    if (isText(element)) {
        return <span>{element.text}</span>;
    } else if (isCode(element)) {
        // eslint-disable-next-line react/no-unknown-property
        return <code spellcheck={false}>{element.code}</code>;
    } else if (isEmphasis(element)) {
        return <em>{renderElements(children)}</em>;
    } else if (isStrong(element)) {
        return <strong>{renderElements(children)}</strong>;
    } else if (isStrikethrough(element)) {
        return <del>{renderElements(children)}</del>;
    } else if (isLink(element)) {
        return <Link link={element.link} cxn={children} />;
    } else if (isListItem(element)) {
        return <li>{renderElements(children)}</li>;
    } else if (isTaskListMarker(element)) {
        return <TaskListMarker checked={element.taskListMarker.checked} />;
    } else if (isFootnoteReference(element)) {
        return (
            <sup className="footnote-reference">
                <a href={`#${element.footnoteReference}`}>{element.footnoteReference}</a>
            </sup>
        );
    }
    assertCondition(false, `Unsupported inline element!: ${element}`);
}

interface ContentElementProps {
    element: Element;
}
export default function ContentElement(props: ContentElementProps): h.JSX.Element {
    // TODO: element components should register with the editor (via a context?)
    //    (this will be used when applying mutations)
    const el = props.element;
    if (isBlockElement(el.element)) {
        return <BlockElement element={el} />;
    } else if (isInlineElement(el.element)) {
        return <InlineElement element={el} />;
    }
    assertCondition(false, `Unsupported element!: ${el.element}`);
}
