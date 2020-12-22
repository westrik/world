import { h } from 'preact';
import { useContext, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import { TextField } from '~components/InputFields';
import { ApiTask } from '~models/Task';

import createTask from './createTask';
import SubmitButton from '~components/SubmitButton';

interface Props {
    onSubmit: (newTask: ApiTask) => void;
}

export default function TaskCreateForm(props: Props): h.JSX.Element {
    const [description, setDescription] = useState('');
    const authContext = useContext(Auth);
    return (
        <form>
            <TextField
                labelText="What's next?"
                placeholderText={description}
                onChange={(e): void => {
                    setDescription((e.target as HTMLInputElement).value);
                }}
            />
            <SubmitButton
                text="Create task"
                onButtonPress={async () => {
                    if (description) {
                        const newTask = await createTask(authContext, { description });
                        // TODO: handle error
                        props.onSubmit(newTask!);
                        setDescription('');
                    }
                }}
            />
        </form>
    );
}
