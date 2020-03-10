import { h } from 'preact';
import { useContext, useState } from 'preact/hooks';

import { Task } from '~models/Task';
import Auth from '~auth/AuthContext';
import updateTask from '~tasks/updateTask';

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
        e.preventDefault();
        e.stopPropagation();
        toggleEditing();
        const newDescription = (e.target as HTMLInputElement).value;
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
            >
                {!editing ? (
                    <span
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
                        onKeyDown={(e): void => {
                            if (e.key === 'Enter' && !e.shiftKey) {
                                handleSetContent(e);
                            }
                        }}
                        onBlur={handleSetContent}
                    />
                )}
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
                            {...childTask}
                        />
                    ))}
                </ul>
            ) : null}

            {/*<div className={false ? 'drag after' : ''} />*/}
        </li>
    ) : null;
}
