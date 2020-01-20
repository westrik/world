import { API_HOST } from '../config';
import { APITask } from './TasksListing';

export default async function createTask(token: string, content: string): Promise<APITask> {
    const response = await fetch(`${API_HOST}/task`, {
        body: JSON.stringify({ content }),
        headers: {
            Authorization: token,
            'Content-Type': 'application/json',
        },
        method: 'POST',
    });
    return await response.json();
}
