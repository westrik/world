import { ApiTask, Task } from '~models/Task';

const LIST_ROOT = 'LIST_ROOT';

type TaskIdToTasksMap = { [taskId: string]: Array<ApiTask> };

// TOOD: refactor this & add simple tests
export default function mapTaskListToTaskTree(
    tasks: Array<ApiTask>,
    taskIdToChildApiTasks?: TaskIdToTasksMap,
): Array<Task> {
    const taskIdToApiTask = tasks.reduce<Map<string, ApiTask>>(function (
        taskIdToTask: Map<string, ApiTask>,
        task: ApiTask,
    ) {
        // TODO: assert(!(task.id in taskIdToTask));
        taskIdToTask.set(task.id, task);
        return taskIdToTask;
    },
    new Map<string, ApiTask>());

    function computeTaskToChildMap(): TaskIdToTasksMap {
        const _taskIdToChildApiTasks: TaskIdToTasksMap = {};
        tasks.forEach(function (task: ApiTask) {
            const parentId = task.parentId ? task.parentId : LIST_ROOT;
            if (!_taskIdToChildApiTasks[parentId]) {
                _taskIdToChildApiTasks[parentId] = [];
            }
            _taskIdToChildApiTasks[parentId].push(task);
        });
        return _taskIdToChildApiTasks;
    }
    const taskIdToChildren = taskIdToChildApiTasks ? taskIdToChildApiTasks : computeTaskToChildMap();

    return tasks
        .filter((task) => !task.parentId || !taskIdToApiTask.get(task.parentId))
        .map((task) => {
            return {
                ...task,
                childTasks: mapTaskListToTaskTree(taskIdToChildren[task.id] || [], taskIdToChildren), // TODO: look up childTasks
            };
        });
}
