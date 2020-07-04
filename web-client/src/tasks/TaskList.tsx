import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import Container from '~components/Container';
import Header from '~components/Header';
import ListContainer from '~components/ListContainer';
import LoadingSpinner from '~components/LoadingSpinner';
import { ApiTask, Task } from '~models/Task';

import listTasks from './listTasks';
import NewTaskForm from './NewTaskForm';
import TaskRow from './TaskRow';

function TaskList(): h.JSX.Element {
    const [tasks, setTasks] = useState<Array<Task> | null>(null);
    const authContext = useContext(Auth);

    useEffect(() => {
        if (!tasks) {
            listTasks(authContext, (tasks) => {
                if (tasks) {
                    setTasks(tasks);
                } else {
                    setTasks([]);
                }
            });
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
        //   update all positions of tasks in between start position and end position
        console.log(e);
    }

    function handleDragStart(e: Event): void {
        // TODO: reset current drag target to current position
        console.log(e);
    }

    // TODO: set up keyboard event handlers (up/down, etc.)

    return (
        <Container>
            <Header title="tasks" fixed={true}>
                <button type="button" className="btn btn-sm btn-outline-secondary">
                    show completed
                </button>
                <button type="button" className="btn btn-sm btn-outline-secondary">
                    generate report
                </button>
            </Header>

            {tasks ? (
                <ListContainer className="tasks">
                    {tasks.map((task: Task, key: number) => (
                        <TaskRow
                            key={key}
                            handleDragOver={handleDragOver}
                            handleDragStart={handleDragStart}
                            handleDragEnd={handleDragEnd}
                            onCreateTask={(newTask: ApiTask): void => {
                                setTasks([...(tasks as Array<Task>), { ...newTask, childTasks: [] } as Task]);
                            }}
                            {...task}
                        />
                    ))}
                </ListContainer>
            ) : (
                <LoadingSpinner />
            )}
        </Container>
    );
}

export default TaskList;
