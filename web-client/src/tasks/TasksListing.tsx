import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';
import Auth from '../auth/AuthContext';
import { API_HOST } from '../config';
import Container from '../components/Container';
import Header from '../components/Header';
import Task from './Task';
import ListContainer from '../components/ListContainer';
import NewTaskForm from './NewTaskForm';

export interface APITask {
    content: string;
}

interface GetTasksResponse {
    error: string | null;
    items: Array<APITask>;
}

function TasksListing(): h.JSX.Element {
    const [tasks, setTasks] = useState<Array<APITask> | null>(null);
    const authContext = useContext(Auth);

    async function getTasks(): Promise<void> {
        const response = await fetch(`${API_HOST}/task`, {
            headers: {
                // TODO: redirect to /login if authToken is expired / null
                Authorization: authContext.authToken!,
                'Content-Type': 'application/json',
            },
            method: 'GET',
        });
        const resp = (await response.json()) as GetTasksResponse;
        if (resp.items) {
            setTasks(resp.items);
        } else {
            console.log(resp.error);
        }
    }

    useEffect(() => {
        if (!tasks) {
            getTasks();
        }
    });

    return (
        <Container>
            <Header title="tasks">
                <NewTaskForm />
                <button type="button" className="btn btn-sm btn-outline-secondary">
                    show personal
                </button>
                <button type="button" className="btn btn-sm btn-outline-secondary">
                    show completed
                </button>
                <button type="button" className="btn btn-sm btn-outline-secondary">
                    generate report
                </button>
            </Header>

            {tasks ? (
                <ListContainer>
                    {tasks.map((item, key) => (
                        <Task key={key} description={item.content} />
                    ))}
                </ListContainer>
            ) : (
                <div className="spinner-border mx-auto" role="status">
                    <span className="sr-only">Loading...</span>
                </div>
            )}
        </Container>
    );
}

export default TasksListing;