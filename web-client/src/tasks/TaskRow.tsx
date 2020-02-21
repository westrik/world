import { h } from 'preact';
import { useState } from 'preact/hooks';

import { Task } from '~models/Task';

export interface Props extends Task {
    handleDragOver: (e: Event) => void;
    handleDragEnd: (e: Event) => void;
    handleDragStart: (e: Event) => void;
}

export default function TaskRow(props: Props): h.JSX.Element | null {
    const [editing, setEditing] = useState(false);
    const [deleted, setDeleted] = useState(false);
    const [description, setDescription] = useState(props.description);
    const [completed, setCompleted] = useState(Boolean(props.completedAt));

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
        // TODO: PUT call
    }

    function handleSetContent(e: Event): void {
        e.preventDefault();
        e.stopPropagation();
        const newContent = (e.target as HTMLInputElement).value;
        if (newContent) {
            setDescription(newContent);
            toggleEditing();
            // TODO: PUT call
        } else {
            setDeleted(true);
            // TODO: DELETE call
        }
    }

    function handleToggleCompleted(): void {
        setCompleted(!completed);
    }

    // TODO: [shift]-[up/down] drags task up or down by one
    // TODO: add task hover/pre-focus state
    // TODO: resolve tags to chips

    return !deleted ? (
        <li className="task" style="font-size: 1.5rem; min-height: 2em;">
            <input
                checked={completed}
                id={checkboxId}
                type="checkbox"
                className="mt-3"
                onChange={handleToggleCompleted}
            />
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
                        className={completed ? 'completed' : ''}
                    >
                        {description}
                    </span>
                ) : (
                    <input
                        ref={(ref): void | null => ref && ref.focus()}
                        type="text"
                        value={description}
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

            {props.childTasks.length > 0 ? (
                <ul className="child_task_list">
                    {/*<li className="add_child_task">+</li>*/}
                    {props.childTasks.map((childTask: Task, key: number) => (
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
    ) : null;
}
