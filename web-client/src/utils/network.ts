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

export class RequestError extends Error {
    response?: Response;
    errorMessage?: string | null;

    constructor(response?: Response, responseJson?: ApiResponse) {
        let message;
        if (response) {
            message = `Received status code ${response.status}: ${responseJson?.error ?? 'Unknown error'}`;
        } else {
            message = 'API call failed';
        }
        super(message);

        // Maintains proper stack trace for where our error was thrown (only available on V8)
        if (Error.captureStackTrace) {
            Error.captureStackTrace(this, RequestError);
        }
        this.name = 'RequestError';
        this.response = response;
        this.errorMessage = responseJson?.error;
    }
}

// TODO: refactor `request` to yield promises. If a request fails w/ code >500, it should be automatically retried -
//  but the caller should receive an update with the error.

// TODO: use service worker to cache responses for some/all API responses (then auto-refresh on cache hit)

export async function request<RequestT = null, ResponseT extends ApiResponse = ApiResponse>(
    method: RequestMethod,
    endpoint: string,
    authContext: AuthContext,
    body?: RequestT,
    credentials?: 'include' | 'omit' | 'same-origin',
): Promise<ResponseT> {
    // TODO: implement retries for 500+ status codes (w/ exponential backoff?)
    let response: Response;
    // eslint-disable-next-line no-useless-catch
    try {
        response = await fetch(`${API_HOST}${endpoint}`, {
            body: body ? JSON.stringify(body) : undefined,
            credentials,
            headers: {
                // TODO: redirect to /login if authToken is expired / null
                Authorization: authContext.authToken!,
                'Content-Type': 'application/json',
            },
            method,
        });
    } catch (e) {
        // TODO: retry on e.g. NetworkError
        throw e;
    }
    const responseJson: ResponseT = await response.json();
    if (response.status > 400) {
        // TODO: redirect to login on auth error
        console.error('Request error', responseJson.error);
        // TODO: trigger a toast
        throw new RequestError(response, responseJson);
    }
    return responseJson;
}
