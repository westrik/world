import { request, RequestMethod } from '~utils/network';
import { GetTasksResponse } from '~tasks/TaskList';
import { AuthContext } from '~auth/AuthContext';
import { Task } from '~models/Task';
import mapTaskListToTaskTree from '~tasks/mapTaskListToTaskTree';

export default async function listTasks(
    authContext: AuthContext,
    onReceiveResponse: (tasks: Array<Task>) => void,
): Promise<void> {
    // TODO: check + save to localStorage
    const response = await request<null, GetTasksResponse>(RequestMethod.GET, '/task', authContext);
    // TODO: handle errors
    const taskTree = mapTaskListToTaskTree(response.tasks);
    onReceiveResponse(taskTree);
}
