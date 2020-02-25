import { API_HOST } from '~config';
import { APITask, APITaskResponse } from '~models/Task';

export default async function createTask(token: string, description: string): Promise<APITask | null> {
    const response = await fetch(`${API_HOST}/task`, {
        body: JSON.stringify({ description }),
        headers: {
            Authorization: token,
            'Content-Type': 'application/json',
        },
        method: 'POST',
    });
    const responseJson: APITaskResponse = await response.json();
    // TODO: handle error
    return responseJson.task;
}
