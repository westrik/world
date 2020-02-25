import { h } from 'preact';
import { useContext, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import { APITask } from '~models/Task';

import createTask from './createTask';

interface Props {
    onSubmit: (newTask: APITask) => void;
}

export default function NewTaskForm(props: Props): h.JSX.Element {
    const [description, setDescription] = useState('');
    const authContext = useContext(Auth);
    return (
        <form
            className="form-group form-inline"
            onSubmit={async (e): Promise<void> => {
                e.preventDefault();
                // TODO: enforce constraints on backend
                if (description) {
                    const newTask = await createTask(authContext.authToken!, description);
                    // TODO: handle error
                    props.onSubmit(newTask!);
                    setDescription('');
                }
            }}
        >
            <input
                type="text"
                className="form-control float-left mr-2"
                style="width: 50%"
                placeholder="what's next?"
                value={description}
                onChange={(e): void => {
                    setDescription((e.target as HTMLInputElement).value);
                }}
            />
            <button type="submit" className="btn btn-sm btn-outline-secondary mr-2">
                create task
            </button>
        </form>
    );
}
