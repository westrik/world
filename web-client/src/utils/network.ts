// TODO: use this to warn before closing window if there are network requests in flight
// window.addEventListener('beforeunload', function(e) {
//     const confirmationMessage = 'waiting for network';
//     // for non-webkit-based browsers:
//     (e || window.event).returnValue = confirmationMessage;
//     // for webkit-based browsers:
//     return confirmationMessage;
// });

import { API_HOST } from '~config';
import { AuthContext } from '~auth/AuthContext';

export interface ApiResponse {
    error: string | null;
}

export enum RequestMethod {
    DELETE = 'DELETE',
    GET = 'GET',
    PATCH = 'PATCH',
    POST = 'POST',
    PUT = 'PUT',
}

export async function request<RequestT = null, ResponseT extends ApiResponse = ApiResponse>(
    method: RequestMethod,
    endpoint: string,
    authContext: AuthContext,
    body?: RequestT,
): Promise<ResponseT | null> {
    const response = await fetch(`${API_HOST}${endpoint}`, {
        body: body ? JSON.stringify(body) : undefined,
        headers: {
            // TODO: redirect to /login if authToken is expired / null
            Authorization: authContext.authToken!,
            'Content-Type': 'application/json',
        },
        method,
    });
    const responseJson: ResponseT = await response.json();
    if (responseJson.error) {
        // TODO: redirect to login on auth error
        console.error('Request error', responseJson.error);
        // TODO: otherwise, display an error to the user
        // TODO: implement retries for 500+ status codes
        return null;
    }
    return responseJson;
}
