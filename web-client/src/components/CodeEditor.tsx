import * as CodeMirror from 'codemirror';
import { Editor, EditorFromTextArea } from 'codemirror';
import 'codemirror/mode/markdown/markdown';
import 'codemirror/keymap/vim';
import { h } from 'preact';
import { useEffect, useRef, useState } from 'preact/hooks';

import { Maybe } from '~utils/types';

export enum EditorLanguage {
    MARKDOWN = 'markdown',
}

interface CodeEditorProps {
    language?: EditorLanguage;
    content: Maybe<string>;
    onChange: (content: string) => void;
}

export default function CodeEditor(props: CodeEditorProps): h.JSX.Element {
    const textareaNode = useRef<HTMLTextAreaElement>(null);
    const [codeMirror, setCodeMirror] = useState<EditorFromTextArea | null>(null);

    // Note: this component does not restart CodeMirror when props.content changes
    // TODO: fix this ^
    useEffect(() => {
        if (codeMirror) {
            return (): void => {
                codeMirror.toTextArea();
            };
        } else {
            const newCodeMirror = CodeMirror.fromTextArea(textareaNode.current, {
                lineNumbers: true,
                mode: props.language,
                keyMap: 'vim',
                theme: 'westrikworld',
            });
            newCodeMirror.on('change', (editor: Editor): void => {
                const currentContent = editor.getValue();
                props.onChange(currentContent);
            });
            setCodeMirror(newCodeMirror);
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [codeMirror]);

    return (
        <div className="code-editor">
            <textarea ref={textareaNode}>{props.content}</textarea>
        </div>
    );
}
