import * as CodeMirror from 'codemirror';
import { EditorFromTextArea } from 'codemirror';
import 'codemirror/mode/markdown/markdown';
import 'codemirror/keymap/vim';
import { h } from 'preact';
import { useEffect, useRef, useState } from 'preact/hooks';

export enum EditorLanguage {
    MARKDOWN = 'markdown',
}

interface CodeEditorProps {
    language?: EditorLanguage;
    content?: string;
}

export default function CodeEditor(props: CodeEditorProps): h.JSX.Element {
    const textareaNode = useRef<HTMLTextAreaElement>(null);
    const [codeMirror, setCodeMirror] = useState<EditorFromTextArea | null>(null);

    useEffect(() => {
        if (codeMirror) {
            return (): void => {
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
    }, [codeMirror, props.language]);

    return (
        <div className="code-editor">
            <textarea ref={textareaNode}>{props.content}</textarea>
        </div>
    );
}
