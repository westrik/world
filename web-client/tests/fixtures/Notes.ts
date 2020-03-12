import { HeadingType, LinkType, Note } from '~models/Note';

export const NOTE_FIXTURE: Note = {
    apiId: 'note_xsOqTy6c',
    createdAt: new Date('2020-03-12T05:28:44.562426Z'),
    updatedAt: new Date('2020-03-12T05:28:44.562426Z'),
    content: {
        elements: [
            {
                children: [
                    {
                        children: null,
                        element: {
                            text: 'Test Document',
                        },
                    },
                ],
                element: {
                    header: HeadingType.H1,
                },
            },
            {
                children: [
                    {
                        children: null,
                        element: {
                            text:
                                'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.',
                        },
                    },
                ],
                element: 'p',
            },
            {
                children: [
                    {
                        children: null,
                        element: {
                            text: 'print("Hello world")',
                        },
                    },
                    {
                        children: null,
                        element: {
                            text: '\n',
                        },
                    },
                ],
                element: {
                    codeBlock: {
                        language: 'py',
                    },
                },
            },
            {
                children: [
                    {
                        children: null,
                        element: {
                            text:
                                'Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum ',
                        },
                    },
                    {
                        children: [
                            {
                                children: null,
                                element: {
                                    text: 'test link',
                                },
                            },
                        ],
                        element: {
                            link: {
                                destinationUrl: 'https://google.com',
                                title: '',
                                type: LinkType.Inline,
                            },
                        },
                    },
                    {
                        children: null,
                        element: {
                            text:
                                ' dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. ',
                        },
                    },
                ],
                element: 'p',
            },
            {
                children: [
                    {
                        children: [
                            {
                                children: null,
                                element: {
                                    text: 'test image',
                                },
                            },
                        ],
                        element: {
                            image: {
                                destinationUrl: 'https://placedog.net/1000',
                                title: 'placedog',
                                type: LinkType.Inline,
                            },
                        },
                    },
                ],
                element: 'p',
            },
        ],
        schemaVersion: '0.1.x',
    },
};
