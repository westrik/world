import { h } from 'preact';
import { useContext, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import { authenticate, SignInResponse } from '~auth/authenticate';
import { SITE_NAME } from '~config';

// @ts-ignore
import logo from '../static/img/logo.png';

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
    const [errorMessage, setErrorMessage] = useState('');

    return (
        <div className="sign-in-form">
            <form className="tile">
                <h1>{<img src={logo} alt={SITE_NAME} />}</h1>
                {errorMessage ? (
                    <div className="alert danger" role="alert">
                        {errorMessage}
                    </div>
                ) : null}
                <label htmlFor="inputEmail" className="sr-only">
                    Email address
                </label>
                <input
                    type="email"
                    id="inputEmail"
                    className="form-control"
                    placeholder="Email address"
                    required
                    autoFocus
                    onChange={(e): void => setEmail((e.target as HTMLInputElement).value)}
                />
                <label htmlFor="inputPassword" className="sr-only">
                    Password
                </label>
                <input
                    type="password"
                    id="inputPassword"
                    className="form-control"
                    placeholder="Password"
                    required
                    onChange={(e): void => setPassword((e.target as HTMLInputElement).value)}
                />
                <div className="checkbox">
                    <label>
                        <input
                            type="checkbox"
                            value="remember-me"
                            onClick={(e): void => setRemember((e.target as HTMLInputElement).checked)}
                        />{' '}
                        Remember me
                    </label>
                </div>
                <button
                    onClick={async (event): Promise<void> => {
                        event.preventDefault();
                        setLoading(true);
                        const res = await handleSignIn(email, password, remember);
                        if (!res) {
                            setErrorMessage('Invalid username or password');
                            setLoading(false);
                        }
                    }}
                    className="button lg"
                    type="submit"
                    disabled={isLoading}
                >
                    Sign in
                </button>
            </form>
        </div>
    );
}

export default SignInForm;
