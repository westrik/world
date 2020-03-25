import { h } from 'preact';
import { useContext, useState } from 'preact/hooks';

import { Task } from '~models/Task';
import Auth from '~auth/AuthContext';
import updateTask from '~tasks/updateTask';

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
        toggleEditing();
        const newDescription = (e.target as HTMLSpanElement).innerText;
        if (newDescription && newDescription !== description) {
            setDescription(newDescription);
            updateTask(authContext, props.apiId, { description: newDescription });
        } else if (!newDescription) {
            setDeleted(true);
            // TODO: DELETE call
        }
    }

    function handleToggleCompleted(): void {
        const isCompleted = !completed;
        setCompleted(isCompleted);
        updateTask(authContext, props.apiId, { isCompleted });
    }

    // TODO: [shift]-[up/down] drags task up or down by one
    // TODO: add task hover/pre-focus state
    // TODO: resolve tags to chips

    return !deleted ? (
        <li className="task" style="font-size: 1.5rem; min-height: 2em;">
            {/*<div className={false ? 'drag before' : ''} />*/}
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
            />
            <span
                tabIndex={0}
                contentEditable={true}
                onClick={handleToggle}
                onFocus={handleToggle}
                onKeyDown={(e): void => {
                    if (e.keyCode === 13 && !e.shiftKey) {
                        e.preventDefault();
                        (e.target as HTMLSpanElement).blur();
                        // TODO: focus the next task
                    }
                }}
                onBlur={(e): void => {
                    handleSetContent(e);
                }}
                className={completed ? 'completed' : ''}
            >
                {description}
            </span>

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
