import { h } from 'preact';
import { useState } from 'preact/hooks';

export interface Props {
    description: string;
    completed?: boolean;
    position: number;
    handleDragOver: (e: Event) => void;
    handleDragEnd: (e: Event) => void;
    handleDragStart: (e: Event) => void;
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
        const newContent = (e.target as HTMLInputElement).value;
        if (newContent) {
            setContent(newContent);
        }
        toggleEditing();
    }

    // TODO: [shift]-[up/down] drags task up or down by one

    return (
        <li className="task" style="font-size: 1.5em; height: 1.8em;">
            <input checked={props.completed} id={checkboxId} type="checkbox" className="mt-3" />
            <label
                htmlFor={checkboxId}
                draggable={true}
                onDragStart={props.handleDragStart}
                onDragOver={props.handleDragOver}
                onDragEnd={props.handleDragEnd}
                style="width:100%"
            >
                {!editing ? (
                    <span style="display: inline-block; width:100%" tabIndex={0} onClick={handleToggle} onFocus={handleToggle}>
                        {content}
                    </span>
                ) : (
                    <input
                        ref={(ref): void | null => ref && ref.focus()}
                        type="text"
                        value={content}
                        style="font-size:0.9em; width: 100%"
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
