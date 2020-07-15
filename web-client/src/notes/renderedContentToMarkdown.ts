/* eslint-disable @typescript-eslint/no-use-before-define */

import { Content } from '~models/Note';

import {
    Element,
    HeaderType,
    isBlockElement,
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
    isTaskListMarker,
    isText,
    LinkData,
    TableData,
} from '~/models/Note';
import { assertCondition } from '~utils/asserts';

function renderCodeBlock(language: string | null, children: Array<Element>): string {
    return `\`\`\`${language}\n${renderElements(children)}\`\`\``;
}

function renderHeader(headerType: HeaderType, children: Array<Element>): string {
    const renderedChildren = renderElements(children);
    if (headerType == HeaderType.H1) {
        return `# ${renderedChildren}`;
    } else if (headerType == HeaderType.H2) {
        return `## ${renderedChildren}`;
    } else if (headerType == HeaderType.H3) {
        return `### ${renderedChildren}`;
    } else if (headerType == HeaderType.H4) {
        return `#### ${renderedChildren}`;
    } else if (headerType == HeaderType.H5) {
        return `##### ${renderedChildren}`;
    } else {
        return `##### ${renderedChildren}`;
    }
}

function renderLink(data: LinkData, children: Array<Element>): string {
    const { destinationUrl, title } = data;
    const paddedTitle = title ? ` "${title}"` : null;
    return `[${renderElements(children)}](${destinationUrl}${paddedTitle})`;
}

function renderImage({ destinationUrl, title }: LinkData): string {
    return `![${title}](${destinationUrl} "${title}")`;
}

function renderTaskListMarker(checked: boolean): string {
    return `[${checked ? 'x' : ' '}]`;
}

// TODO:
// function renderTableHeader(element: Element, columnTypes: Array<ColumnType>): string {
//     return '';
// }
//
// function renderTableRow(element: Element, columnTypes: Array<ColumnType>): string {
//     return '';
// }

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function renderTable(_data: TableData, _children: Array<Element>): string {
    return '';
}

function renderBlockElement(el: Element): string {
    const { element, children } = el;
    if (isHtml(element)) {
        // TODO: strip and validate on server-side; set with dangerouslySetInnerHTML
    } else if (isParagraph(element)) {
        return `\n${renderElements(children)}`;
    } else if (isCodeBlock(element)) {
        return renderCodeBlock(element.codeBlock.language, children!);
    } else if (isHeaderElement(element)) {
        return renderHeader(element.header, children!);
    } else if (isImage(element)) {
        return renderImage(element.image);
    } else if (isList(element)) {
        return `\n${renderElements(children)}`;
    } else if (isFootnoteDefinition(element)) {
        // TODO
    } else if (isTable(element)) {
        return renderTable(element.table, children!);
    } else if (isSoftBreak(element)) {
        return '\n';
    } else if (isHardBreak(element)) {
        return '\n\n';
    } else if (isRule(element)) {
        return '\n-------\n';
    }
    assertCondition(false, 'Unsupported block element!');
}

function renderInlineElement(el: Element): string {
    const { element, children } = el;
    if (isText(element)) {
        return element.text;
    } else if (isCode(element)) {
        return `\`${element.code}\``;
    } else if (isEmphasis(element)) {
        return `_${renderElements(children)}_`;
    } else if (isStrong(element)) {
        return `**${renderElements(children)}**`;
    } else if (isStrikethrough(element)) {
        return `~~${renderElements(children)}~~`;
    } else if (isLink(element)) {
        return renderLink(element.link, children!);
    } else if (isListItem(element)) {
        return `- ${renderElements(children)}`;
    } else if (isTaskListMarker(element)) {
        return renderTaskListMarker(element.taskListMarker.checked);
    } else if (isFootnoteReference(element)) {
        return `[^${element.footnoteReference}}]`;
    }
    assertCondition(false, 'Unsupported inline element!');
}

function renderElement(el: Element): string {
    if (isBlockElement(el.element)) {
        return renderBlockElement(el);
    } else if (isInlineElement(el.element)) {
        return renderInlineElement(el);
    }
    assertCondition(false, 'Unsupported element!');
}

function renderElements(cxn: Array<Element> | null): string {
    return cxn ? cxn.map((el) => renderElement(el)).join('\n') : '';
}

export default function renderedContentToMarkdown(content: Content): string {
    return renderElements(content.elements);
}
