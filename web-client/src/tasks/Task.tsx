import { h } from 'preact';
import { useState } from 'preact/hooks';

export interface Props {
    description: string;
    completed?: boolean;
}

export default function Task(props: Props): h.JSX.Element {
    const [editing, setEditing] = useState(false);
    const [content, setContent] = useState(props.description);
    const checkboxId = Math.random()
        .toString(36)
        .substring(2, 15);

    function toggleEditing(): void {
        setEditing(!editing);
    }

    function handleToggle(e: Event): void {
        e.preventDefault();
        e.stopPropagation();
        toggleEditing();
    }

    function handleSetContent(e: Event): void {
        e.preventDefault();
        e.stopPropagation();
        setContent((e.target as HTMLInputElement).value);
        toggleEditing();
    }

    return (
        <li className="task" style="font-size: 1.5em;">
            <input checked={props.completed} id={checkboxId} type="checkbox" className="mt-3" />
            <label htmlFor={checkboxId}>
                {!editing ? (
                    <span tabIndex={0} onClick={handleToggle} onFocus={handleToggle}>
                        {content}
                    </span>
                ) : (
                    <input
                        ref={(inputRef): void | null => inputRef && inputRef.focus()}
                        type="text"
                        value={content}
                        onKeyDown={(e): void => {
                            if (e.key === 'Enter') {
                                handleSetContent(e);
                            }
                        }}
                        onBlur={handleSetContent}
                    />
                )}
            </label>
        </li>
    );
}
