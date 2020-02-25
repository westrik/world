import { API_HOST } from '~config';
import { APITask } from '~models/Task';

interface TaskUpdateSpec {
    description?: string;
    isCompleted?: boolean;
}

export default async function updateTask(token: string, apiId: string, spec: TaskUpdateSpec): Promise<APITask> {
    const response = await fetch(`${API_HOST}/task/${apiId}`, {
        body: JSON.stringify(spec),
        headers: {
            Authorization: token,
            'Content-Type': 'application/json',
        },
        method: 'PUT',
    });
    return await response.json();
}
