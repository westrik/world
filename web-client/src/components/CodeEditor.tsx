import { h } from 'preact';
import * as CodeMirror from 'codemirror';
import 'codemirror/mode/markdown/markdown';
import 'codemirror/keymap/vim';

import { useEffect, useRef, useState } from 'preact/hooks';
import { EditorFromTextArea } from 'codemirror';

export enum EditorLanguage {
    MARKDOWN = 'markdown',
}

interface CodeEditorProps {
    language?: EditorLanguage;
}

export default function CodeEditor(props: CodeEditorProps): h.JSX.Element {
    const textareaNode = useRef<HTMLTextAreaElement>(null);
    const [codeMirror, setCodeMirror] = useState<EditorFromTextArea | null>(null);

    useEffect(() => {
        if (codeMirror) {
            return () => {
                codeMirror.toTextArea();
            };
        } else {
            setCodeMirror(
                CodeMirror.fromTextArea(textareaNode.current, {
                    lineNumbers: true,
                    mode: props.language,
                    keyMap: 'vim',
                }),
            );
        }
    }, [codeMirror]);

    return (
        <div className="code-editor">
            <textarea ref={textareaNode}>Hello world!</textarea>
        </div>
    );
}
