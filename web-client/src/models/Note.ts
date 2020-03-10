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
    | HeadingElement
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
export interface HeadingElement {
    heading: HeadingType;
}
export enum HeadingType {
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
