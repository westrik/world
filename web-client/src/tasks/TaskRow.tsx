import { h } from 'preact';
import { useContext, useState } from 'preact/hooks';

import { Task } from '~models/Task';
import Auth from '~auth/AuthContext';
import updateTask from '~tasks/updateTask';
import { randomIdentifier } from '~utils/identifier';

export interface Props extends Task {
    handleDragOver: (e: Event) => void;
    handleDragEnd: (e: Event) => void;
    handleDragStart: (e: Event) => void;
    onCreateTask: (newTask: Task) => void;
}

export default function TaskRow(props: Props): h.JSX.Element | null {
    const [editing, setEditing] = useState(false);
    const [deleted, setDeleted] = useState(false);
    const [description, setDescription] = useState(props.description);
    const [completed, setCompleted] = useState(Boolean(props.completedAt));
    const authContext = useContext(Auth);

    const checkboxId = randomIdentifier();

    function toggleEditing(): void {
        setEditing(!editing);
    }

    function handleToggle(e: Event): void {
        e.preventDefault();
        e.stopPropagation();
        toggleEditing();
    }

    function handleSetContent(e: Event): void {
        toggleEditing();
        const newDescription = (e.target as HTMLSpanElement).innerText;
        if (newDescription && newDescription !== description) {
            setDescription(newDescription);
            updateTask(authContext, props.id, { description: newDescription });
        } else if (!newDescription) {
            setDeleted(true);
            // TODO: DELETE call
        }
    }

    function handleToggleCompleted(): void {
        const isCompleted = !completed;
        setCompleted(isCompleted);
        updateTask(authContext, props.id, { isCompleted });
    }

    // TODO: [shift]-[up/down] drags task up or down by one
    // TODO: add task hover/pre-focus state
    // TODO: resolve tags to chips

    return !deleted ? (
        <li className="task" style="font-size: 1.5rem; min-height: 2em;">
            {/*<div className={false ? 'drag before' : ''} />*/}
            <label
                htmlFor={checkboxId}
                draggable={!editing}
                onDragStart={props.handleDragStart}
                onDragOver={props.handleDragOver}
                onDragEnd={props.handleDragEnd}
            >
                <input
                    checked={completed}
                    id={checkboxId}
                    type="checkbox"
                    className="mt-3"
                    onChange={handleToggleCompleted}
                />
                <span
                    role="checkbox"
                    aria-checked={completed}
                    tabIndex={0}
                    contentEditable={true}
                    onClick={handleToggle}
                    onFocus={handleToggle}
                    onKeyDown={(e): void => {
                        if (e.code === 'Escape' && !e.shiftKey) {
                            e.preventDefault();
                            (e.target as HTMLSpanElement).blur();
                            // TODO: focus the next task
                        }
                    }}
                    onBlur={handleSetContent}
                    /* TODO: handle contenteditable redo/undo?
                         or: implement custom undo/redo */
                    className={completed ? 'completed' : ''}
                >
                    {description}
                </span>
            </label>

            {props.childTasks.length > 0 ? (
                <ul className="child_task_list">
                    <li className="drag child" />
                    {/*<li className="add_child_task">+</li>*/}
                    {props.childTasks.map((childTask: Task, key: number) => (
                        <TaskRow
                            key={key}
                            handleDragOver={props.handleDragOver}
                            handleDragEnd={props.handleDragEnd}
                            handleDragStart={props.handleDragStart}
                            onCreateTask={props.onCreateTask}
                            {...childTask}
                        />
                    ))}
                </ul>
            ) : null}

            {/*<div className={false ? 'drag after' : ''} />*/}
        </li>
    ) : null;
}
