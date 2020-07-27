import { ApiTask, ApiTaskResponse } from '~models/Task';
import { AuthContext } from '~auth/AuthContext';
import { request, RequestMethod } from '~utils/network';

interface TaskCreateSpec {
    description: string;
}

export default async function createTask(
    authContext: AuthContext,
    createSpec: TaskCreateSpec,
): Promise<ApiTask | null> {
    const responseJson = await request<TaskCreateSpec, ApiTaskResponse>(
        RequestMethod.POST,
        '/task/',
        authContext,
        createSpec,
    );
    // TODO: improve error-handling
    return responseJson?.task ?? null;
}
