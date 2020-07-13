import { h } from 'preact';
import { useContext, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import { authenticate, SignInResponse } from '~auth/authenticate';
import { SITE_NAME } from '~config';

import { EmailField, PasswordField } from '~components/InputFields';
import LoadingSpinner from '~components/LoadingSpinner';
import SubmitButton from '~components/SubmitButton';
import Toggle from '~components/Toggle';

function SignInForm(): h.JSX.Element {
    const authContext = useContext(Auth);
    return (
        <UnconnectedSignInForm
            handleSignIn={async (email, password, remember): Promise<boolean> => {
                let res: SignInResponse | null = null;
                try {
                    res = await authenticate(email, password);
                    authContext.handleSignIn(res.session, remember);
                    return true;
                } catch {
                    return false;
                }
            }}
        />
    );
}

export function UnconnectedSignInForm({
    handleSignIn,
}: {
    handleSignIn: (email: string, password: string, remember: boolean) => Promise<boolean>;
}): h.JSX.Element {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [remember, setRemember] = useState(false);
    const [isLoading, setLoading] = useState(false);
    const [errorMessage, setErrorMessage] = useState<string | null>(null);

    return (
        <div className="sign-in-form">
            <div>
                {errorMessage ? (
                    <div className={`alert danger ${isLoading ? 'fade-out' : 'fade-in'}`} role="alert">
                        {errorMessage}
                    </div>
                ) : null}
                {isLoading ? <LoadingSpinner className="fade-in" /> : null}
                <form className={isLoading ? 'fade-out' : 'fade-in'}>
                    <h1>
                        <span className="sr-only">Sign-in for</span>
                        {SITE_NAME}
                    </h1>
                    <EmailField
                        labelText="Email address"
                        onChange={(event: Event): void => {
                            setEmail((event.target as HTMLInputElement).value);
                        }}
                    />
                    <PasswordField
                        labelText="Password"
                        onChange={(event: Event): void => {
                            setPassword((event.target as HTMLInputElement).value);
                        }}
                    />
                    <Toggle
                        labelText="Remember me"
                        onChange={(event): void => {
                            setRemember((event.target as HTMLInputElement).checked);
                        }}
                    />
                    <SubmitButton
                        text="Sign in"
                        disabled={isLoading}
                        onButtonPress={async (event: Event): Promise<void> => {
                            event.preventDefault();
                            setLoading(true);
                            const res = await handleSignIn(email, password, remember);
                            if (!res) {
                                setErrorMessage('Invalid username or password');
                                setLoading(false);
                            }
                        }}
                    />
                </form>
            </div>
        </div>
    );
}

export default SignInForm;
