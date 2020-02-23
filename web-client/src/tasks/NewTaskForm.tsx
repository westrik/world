import { h } from 'preact';
import { useContext, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';

import createTask from './createTask';

interface Props {
    onSubmit: (description: string) => void;
}

export default function NewTaskForm(props: Props): h.JSX.Element {
    const [newTaskContent, setNewTaskContent] = useState('');
    const authContext = useContext(Auth);
    return (
        <form
            className="form-group form-inline"
            onSubmit={(e): void => {
                e.preventDefault();
                // TODO: enforce constraints on backend
                if (newTaskContent) {
                    createTask(authContext.authToken!, newTaskContent);
                    props.onSubmit(newTaskContent);
                    setNewTaskContent('');
                }
            }}
        >
            <input
                type="text"
                className="form-control float-left mr-2"
                style="width: 50%"
                placeholder="what's next?"
                value={newTaskContent}
                onChange={(e): void => {
                    setNewTaskContent((e.target as HTMLInputElement).value);
                }}
            />
            <button type="submit" className="btn btn-sm btn-outline-secondary mr-2">
                create task
            </button>
        </form>
    );
}
