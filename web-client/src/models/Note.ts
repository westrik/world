import { Tag } from './Tag';
import { Resource } from './Resource';
import { ApiResponse } from '~utils/network';

export interface ApiNoteResponse extends ApiResponse {
    note: Note | null;
}

export interface ApiNoteSummary {
    apiId: string;
    createdAt: Date;
    updatedAt: Date;
}

// export interface ApiNote extends ApiNoteSummary {
//     content: Content;
// }

export interface Note extends ApiNoteSummary {
    content?: Content;
    tags?: Array<Tag>;
    resources?: Array<Resource>;
}

/* Content schema: */

export interface Content {
    elements: Array<Element>;
    schemaVersion: string;
}

export interface Element {
    element: ElementData;
    children: Array<Element> | null;
}

export type ElementData =
    | TextElement
    | CodeElement
    | HtmlElement
    | ParagraphElement
    | EmphasisElement
    | StrongElement
    | StrikethroughElement
    | HeaderElement
    | LinkElement
    | ImageElement
    | CodeBlockElement
    | ListElement
    | ListItemElement
    | TaskListMarkerElement
    | FootnoteDefinitionElement
    | FootnoteReferenceElement
    | TableElement
    | TableHeadElement
    | TableRowElement
    | TableCellElement
    | SoftBreakElement
    | HardBreakElement
    | RuleElement;

export interface TextElement {
    text: string;
}
export interface CodeElement {
    code: string;
}
export interface HtmlElement {
    html: string;
}
export type ParagraphElement = 'p';
export type EmphasisElement = 'em';
export type StrongElement = 'strong';
export type StrikethroughElement = 'strike';
export interface HeaderElement {
    header: HeaderType;
}
export enum HeaderType {
    H1 = 'h1',
    H2 = 'h2',
    H3 = 'h3',
    H4 = 'h4',
    H5 = 'h5',
    H6 = 'h6',
}
export interface LinkElement {
    link: LinkData;
}
export enum LinkType {
    Inline = 'inline',
    Reference = 'reference',
    Collapsed = 'collapsed',
    Shortcut = 'shortcut',
    Autolink = 'autolink',
    Email = 'email',
}
export interface LinkData {
    type: LinkType;
    destinationUrl: string;
    title: string;
}
export interface ImageElement {
    image: LinkData;
}
export interface CodeBlockElement {
    codeBlock: CodeBlockData;
}
export interface CodeBlockData {
    language: string | null;
}
export interface ListElement {
    list: ListData;
}
export interface ListData {
    numberOfFirstItem: number | null;
}
export type ListItemElement = 'item';
export interface TaskListMarkerElement {
    taskListMarker: TaskListMarkerData;
}
export interface TaskListMarkerData {
    checked: boolean;
}
export interface FootnoteDefinitionElement {
    footnoteDefinition: string;
}
export interface FootnoteReferenceElement {
    footnoteReference: string;
}
export interface TableElement {
    table: TableData;
}
export interface TableData {
    columnTypes: Array<ColumnType>;
}
export enum ColumnType {
    Unaligned = 'none',
    LeftAligned = 'left',
    CenterAligned = 'center',
    RightAligned = 'right',
}
export type TableHeadElement = 'tableHead';
export type TableRowElement = 'tableRow';
export type TableCellElement = 'tableCell';
export type SoftBreakElement = 'softBreak';
export type HardBreakElement = 'hardBreak';
export type RuleElement = 'rule';

function elementDataHasProperty(element: ElementData, property: string): boolean {
    return Object.prototype.hasOwnProperty.call(element, property);
}
export function isText(el: ElementData): el is TextElement {
    return elementDataHasProperty(el, 'text');
}
export function isCode(el: ElementData): el is CodeElement {
    return elementDataHasProperty(el, 'code');
}
export function isHtml(el: ElementData): el is HtmlElement {
    return elementDataHasProperty(el, 'html');
}
export function isParagraph(el: ElementData): el is ParagraphElement {
    return el == 'p';
}
export function isEmphasis(el: ElementData): el is EmphasisElement {
    return el == 'em';
}
export function isStrong(el: ElementData): el is StrongElement {
    return el == 'strong';
}
export function isStrikethrough(el: ElementData): el is StrikethroughElement {
    return el == 'strike';
}
export function isHeaderElement(el: ElementData): el is HeaderElement {
    return elementDataHasProperty(el, 'header');
}
export function isLink(el: ElementData): el is LinkElement {
    return elementDataHasProperty(el, 'link');
}
export function isImage(el: ElementData): el is ImageElement {
    return elementDataHasProperty(el, 'image');
}
export function isCodeBlock(el: ElementData): el is CodeBlockElement {
    return elementDataHasProperty(el, 'codeBlock');
}
export function isList(el: ElementData): el is ListElement {
    return elementDataHasProperty(el, 'list');
}
export function isListItem(el: ElementData): el is ListItemElement {
    return elementDataHasProperty(el, 'listItem');
}
export function isTaskListMarker(el: ElementData): el is TaskListMarkerElement {
    return elementDataHasProperty(el, 'taskListMarker');
}
export function isFootnoteDefinition(el: ElementData): el is FootnoteDefinitionElement {
    return elementDataHasProperty(el, 'footnoteDefinition');
}
export function isFootnoteReference(el: ElementData): el is FootnoteReferenceElement {
    return elementDataHasProperty(el, 'footnoteReference');
}
export function isTable(el: ElementData): el is TableElement {
    return elementDataHasProperty(el, 'table');
}
export function isTableHead(el: ElementData): el is TableHeadElement {
    return el == 'tableHead';
}
export function isTableRow(el: ElementData): el is TableRowElement {
    return el == 'tableRow';
}
export function isTableCell(el: ElementData): el is TableCellElement {
    return el == 'tableCell';
}
export function isSoftBreak(el: ElementData): el is SoftBreakElement {
    return el == 'softBreak';
}
export function isHardBreak(el: ElementData): el is HardBreakElement {
    return el == 'hardBreak';
}
export function isRule(el: ElementData): el is RuleElement {
    return el == 'rule';
}
