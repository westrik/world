import { ApiResponse, request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';
import { ApiTask, Task } from '~models/Task';
import mapTaskListToTaskTree from '~tasks/mapTaskListToTaskTree';

export interface GetTasksResponse extends ApiResponse {
    tasks: Array<ApiTask>;
}

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
