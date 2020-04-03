import { h } from 'preact';
import { UnconnectedSignInForm } from '~auth/SignInForm';

export default { title: 'Sign In Form' };

export function signInForm(): h.JSX.Element {
    return (
        <UnconnectedSignInForm
            handleSignIn={(): Promise<boolean> => {
                return Promise.resolve(false);
            }}
        />
    );
}
