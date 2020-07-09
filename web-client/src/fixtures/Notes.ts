import { ColumnType, Element, HeaderType, LinkData, LinkType, ListData, Note } from '~models/Note';

function createText(text: string): Element {
    return {
        element: { text },
        children: null,
    };
}
function createHeader(type: HeaderType, text: string): Element {
    return {
        element: {
            header: type,
        },
        children: [createText(text)],
    };
}
function createParagraph(cxn: Array<Element>): Element {
    return {
        element: 'p',
        children: cxn,
    };
}
function createCodeBlock(language: string, cxn: Array<Element>): Element {
    return {
        element: {
            codeBlock: { language },
        },
        children: cxn,
    };
}
function createLink(link: LinkData, cxn: Array<Element>): Element {
    return {
        element: {
            link,
        },
        children: cxn,
    };
}
function createImage(image: LinkData): Element {
    return {
        element: {
            image,
        },
        children: null,
    };
}
function createList(listData: ListData, cxn: Array<Element>): Element {
    return {
        element: {
            list: listData,
        },
        children: cxn,
    };
}
function createListItem(cxn: Array<Element>): Element {
    return {
        element: 'listItem',
        children: cxn,
    };
}
function createTaskListMarker(checked: boolean): Element {
    return {
        element: {
            taskListMarker: {
                checked,
            },
        },
        children: null,
    };
}

export const NOTE_FIXTURE: Note = {
    apiId: 'note_xsOqTy6c',
    createdAt: new Date('2020-03-12T05:28:44.562426Z'),
    updatedAt: new Date('2020-03-12T05:28:44.562426Z'),
    content: {
        elements: [
            createHeader(HeaderType.H1, 'Example of headline level one'),
            createHeader(HeaderType.H2, 'Headline level two'),
            createHeader(HeaderType.H3, 'Headline level three'),
            createParagraph([
                createText(
                    'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.',
                ),
            ]),
            createCodeBlock('py', [createText('print("Hello world")'), createText('\n')]),
            createParagraph([
                createText(
                    'Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum ',
                ),
                createLink(
                    {
                        destinationUrl: 'https://google.com',
                        title: '',
                        type: LinkType.Inline,
                    },
                    [createText('test link')],
                ),
                createText(
                    ' dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. ',
                ),
            ]),
            createHeader(HeaderType.H4, 'Headline level four'),
            createParagraph([
                createImage({
                    destinationUrl: 'https://placedog.net/1000',
                    title: 'placedog',
                    type: LinkType.Inline,
                }),
            ]),
            createHeader(HeaderType.H5, 'Headline level five'),
            createList(
                {
                    numberOfFirstItem: null,
                },
                [
                    createListItem([createTaskListMarker(false), createText('hello')]),
                    createListItem([createTaskListMarker(true), createText('world')]),
                ],
            ),
            createHeader(HeaderType.H6, 'Headline level six'),
            {
                children: [
                    {
                        children: [
                            {
                                children: [
                                    {
                                        children: null,
                                        element: {
                                            text: 'Column 1',
                                        },
                                    },
                                ],
                                element: 'tableCell',
                            },
                            {
                                children: [
                                    {
                                        children: null,
                                        element: {
                                            text: 'Column 2',
                                        },
                                    },
                                ],
                                element: 'tableCell',
                            },
                            {
                                children: [
                                    {
                                        children: null,
                                        element: {
                                            text: 'Column 3',
                                        },
                                    },
                                ],
                                element: 'tableCell',
                            },
                            {
                                children: [
                                    {
                                        children: null,
                                        element: {
                                            text: 'Column 4',
                                        },
                                    },
                                ],
                                element: 'tableCell',
                            },
                        ],
                        element: 'tableHead',
                    },
                    {
                        children: [
                            {
                                children: [
                                    {
                                        children: null,
                                        element: {
                                            text: 'value 1',
                                        },
                                    },
                                ],
                                element: 'tableCell',
                            },
                            {
                                children: [
                                    {
                                        children: null,
                                        element: {
                                            text: 'value 2',
                                        },
                                    },
                                ],
                                element: 'tableCell',
                            },
                            {
                                children: [
                                    {
                                        children: null,
                                        element: {
                                            text: 'value 3',
                                        },
                                    },
                                ],
                                element: 'tableCell',
                            },
                            {
                                children: [
                                    {
                                        children: null,
                                        element: {
                                            text: 'value 4',
                                        },
                                    },
                                ],
                                element: 'tableCell',
                            },
                        ],
                        element: 'tableRow',
                    },
                    {
                        children: [
                            {
                                children: [
                                    {
                                        children: null,
                                        element: {
                                            text: 'value 5',
                                        },
                                    },
                                ],
                                element: 'tableCell',
                            },
                            {
                                children: [
                                    {
                                        children: null,
                                        element: {
                                            text: 'value 6',
                                        },
                                    },
                                ],
                                element: 'tableCell',
                            },
                            {
                                children: [
                                    {
                                        children: null,
                                        element: {
                                            text: 'value 7',
                                        },
                                    },
                                ],
                                element: 'tableCell',
                            },
                            {
                                children: [
                                    {
                                        children: null,
                                        element: {
                                            text: 'value 8',
                                        },
                                    },
                                ],
                                element: 'tableCell',
                            },
                        ],
                        element: 'tableRow',
                    },
                ],
                element: {
                    table: {
                        columnTypes: [
                            ColumnType.CenterAligned,
                            ColumnType.LeftAligned,
                            ColumnType.RightAligned,
                            ColumnType.Unaligned,
                        ],
                    },
                },
            },
        ],
        schemaVersion: '0.1.x',
    },
};
