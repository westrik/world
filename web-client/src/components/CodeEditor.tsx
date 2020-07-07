import { h } from 'preact';
import { useEffect } from 'preact/hooks';

export enum EditorLanguage {
    MARKDOWN = 'markdown',
}

interface CodeEditorProps {
    language?: EditorLanguage;
}

export default function CodeEditor(props: CodeEditorProps): h.JSX.Element {
    useEffect(() => {
        // TODO: set up CodeMirror

        return () => {
            // TODO: tear down CodeMirror
        };
    }, []);

    return (
        <div className="code-editor">
            <textarea data-language={props.language}>Hello world!</textarea>
        </div>
    );
}
