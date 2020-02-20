import { API_HOST } from '~config';
import { APITask } from '~models/Task';

export default async function createTask(token: string, description: string): Promise<APITask> {
    const response = await fetch(`${API_HOST}/task`, {
        body: JSON.stringify({ description }),
        headers: {
            Authorization: token,
            'Content-Type': 'application/json',
        },
        method: 'POST',
    });
    return await response.json();
}
