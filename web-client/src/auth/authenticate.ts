import { API_HOST } from '~config';
import { Session } from '~models/Session';
import { User } from '~models/User';

export interface SignInResponse {
    user: User;
    session: Session;
}

export async function authenticate(emailAddress: string, password: string): Promise<SignInResponse> {
    const response = await fetch(`${API_HOST}/sign-in`, {
        body: JSON.stringify({ emailAddress, password }),
        headers: {
            'Content-Type': 'application/json',
        },
        method: 'POST',
    });
    return await response.json();
}
