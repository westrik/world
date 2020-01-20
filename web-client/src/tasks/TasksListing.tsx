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
    position: number;
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

    function handleDragOver(e: Event): void {
        // TODO:
        //   get props.position from e.target
        //   set current drag target to props.position
        //   if props.position is less than origin, insert blank space before props.position
        //   if props.position is greater than origin, insert blank space after props.position
        //   hide task being dragged in the list if it's not already
        console.log(e);
    }

    function handleDragEnd(e: Event): void {
        // TODO:
        //   change position of e.target to after current drag target
        //   update all positions of items in between start position and end position
        console.log(e);
    }

    function handleDragStart(e: Event): void {
        // TODO: reset current drag target to current position
        console.log(e);
    }

    // TODO: set up keyboard event handlers (up/down, etc.)

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
                    {tasks
                        .sort((a: APITask, b: APITask): number => a.position - b.position)
                        .map((item: APITask, key: number) => (
                            <Task
                                key={key}
                                position={item.position}
                                handleDragOver={handleDragOver}
                                handleDragStart={handleDragStart}
                                handleDragEnd={handleDragEnd}
                                description={item.content}
                            />
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
