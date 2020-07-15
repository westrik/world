import { ApiTask, ApiTaskResponse } from '~models/Task';
import { request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';

interface TaskUpdateSpec {
    description?: string;
    isCompleted?: boolean;
}

export default async function updateTask(
    authContext: AuthContext,
    taskId: string,
    updateSpec: TaskUpdateSpec,
): Promise<ApiTask | null> {
    const responseJson = await request<TaskUpdateSpec, ApiTaskResponse>(
        RequestMethod.PATCH,
        `/task/${taskId}`,
        authContext,
        updateSpec,
    );
    // TODO: handle error
    return responseJson.task;
}
