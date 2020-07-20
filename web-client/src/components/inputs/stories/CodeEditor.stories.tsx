import { h } from 'preact';

import CodeEditor, { EditorLanguage } from '~components/inputs/CodeEditor';
import PageContainer from '~components/layout/PageContainer';
import Sheet from '~components/layout/Sheet';
import noop from '~utils/noop';

export default { title: 'Markdown Editor' };

export function normal(): h.JSX.Element {
    return (
        <PageContainer>
            <Sheet>
                <h1>Markdown Editor</h1>
                <CodeEditor
                    language={EditorLanguage.MARKDOWN}
                    content={'# Hello world\nthis is a note'}
                    onChange={noop}
                />
            </Sheet>
        </PageContainer>
    );
}
