import { h } from 'preact';

import CodeEditor, { EditorLanguage } from '~components/CodeEditor';
import Container from '~components/Container';
import Sheet from '~components/Sheet';

export default { title: 'Code Editor' };

export function normal(): h.JSX.Element {
    return (
        <Container>
            <Sheet>
                <h1>Code Editor</h1>
                <CodeEditor language={EditorLanguage.MARKDOWN} />
            </Sheet>
        </Container>
    );
}
