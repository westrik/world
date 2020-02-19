import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~/auth/AuthContext';
import { API_HOST } from '~/config';
import Container from '~/components/Container';
import Header from '~/components/Header';
import ListContainer from '~/components/ListContainer';
import { APITask, Task } from '~/models/Task';

import TaskRow from './TaskRow';
import NewTaskForm from './NewTaskForm';

import { API_TASKS } from '~/../tests/fixtures/Tasks';

const LIST_ROOT = 'LIST_ROOT';

type TaskIdToTasksMap = { [taskId: string]: Array<APITask> };

function mapTasksToChildTasks(tasks: Array<APITask>, taskIdToChildAPITasks?: TaskIdToTasksMap): Array<Task> {
    const taskIdToAPITask = tasks.reduce<Map<string, APITask>>(function(
        taskIdToTask: Map<string, APITask>,
        task: APITask,
    ) {
        // TODO: assert(!(task.id in taskIdToTask));
        taskIdToTask.set(task.id, task);
        return taskIdToTask;
    },
    new Map<string, APITask>());

    function computeTaskToChildMap(): TaskIdToTasksMap {
        const _taskIdToChildAPITasks: TaskIdToTasksMap = {};
        tasks.forEach(function(task: APITask) {
            const parentId = task.parentId ? task.parentId : LIST_ROOT;
            if (!_taskIdToChildAPITasks[parentId]) {
                _taskIdToChildAPITasks[parentId] = [];
            }
            _taskIdToChildAPITasks[parentId].push(task);
        });
        return _taskIdToChildAPITasks;
    }
    const taskIdToChildren = taskIdToChildAPITasks ? taskIdToChildAPITasks : computeTaskToChildMap();

    return tasks
        .filter(task => !task.parentId || !taskIdToAPITask.get(task.parentId))
        .map(task => {
            return {
                ...task,
                children: mapTasksToChildTasks(taskIdToChildren[task.id] || [], taskIdToChildren), // TODO: look up children
            };
        });
}

export interface GetTasksResponse {
    error: string | null;
    items: Array<APITask>;
}

function TaskList(): h.JSX.Element {
    const [tasks, setTasks] = useState<Array<Task> | null>(null);
    const authContext = useContext(Auth);

    async function getTasks(): Promise<void> {
        // TODO: check + save to localStorage
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
            setTasks(mapTasksToChildTasks(resp.items));
        } else {
            setTasks([]);
        }
    }

    useEffect(() => {
        if (!tasks) {
            // TODO: re-enable
            // getTasks();
            const x = mapTasksToChildTasks(API_TASKS);
            setTasks(x);
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
                        .sort((a, b): number => a.position! - b.position!)
                        .map((task: Task, key: number) => (
                            <TaskRow
                                key={key}
                                handleDragOver={handleDragOver}
                                handleDragStart={handleDragStart}
                                handleDragEnd={handleDragEnd}
                                {...task}
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

export default TaskList;
