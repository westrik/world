import { h } from 'preact';
import { useState } from 'preact/hooks';
import { Task } from './MockTaskList';

export interface Props extends Task {
    handleDragOver: (e: Event) => void;
    handleDragEnd: (e: Event) => void;
    handleDragStart: (e: Event) => void;
}

export default function TaskRow(props: Props): h.JSX.Element {
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
    // TODO: add task hover/pre-focus state
    // TODO: resolve tags to chips

    return (
        <li className="task" style="font-size: 1.5rem; min-height: 2em;">
            <input checked={props.completed} id={checkboxId} type="checkbox" className="mt-3" />
            <label
                htmlFor={checkboxId}
                draggable={!editing}
                onDragStart={props.handleDragStart}
                onDragOver={props.handleDragOver}
                onDragEnd={props.handleDragEnd}
                style="width:100%"
            >
                {!editing ? (
                    <span
                        style="display: inline-block; width:100%"
                        tabIndex={0}
                        onClick={handleToggle}
                        onFocus={handleToggle}
                    >
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

            {props.children.length > 0 ? (
                <ul style="list-style: none; padding: 0; margin-left: 2em; margin-top: 0.2em;">
                    {props.children.map((childTask: Task, key: number) => (
                        <TaskRow
                            key={key}
                            handleDragOver={props.handleDragOver}
                            handleDragEnd={props.handleDragEnd}
                            handleDragStart={props.handleDragStart}
                            {...childTask}
                        />
                    ))}
                </ul>
            ) : null}
        </li>
    );
}
