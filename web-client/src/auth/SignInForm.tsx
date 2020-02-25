import { h } from 'preact';
import { useContext, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import { authenticate, SignInResponse } from '~auth/authenticate';
import { SITE_NAME } from '~config';

// @ts-ignore
import logo from '../static/img/logo.png';
import '../style/SignInForm.scss';

function SignInForm(): h.JSX.Element {
    const authContext = useContext(Auth);
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [remember, setRemember] = useState(false);
    const [isLoading, setLoading] = useState(false);
    const [errorMessage, setErrorMessage] = useState('');

    return (
        <div className="form-container text-center">
            <form className="form-signin">
                <h1 className={`h3 font-weight-normal ${!errorMessage ? 'mb-3' : null}`}>
                    {<img src={logo} className="mb-3 img-fluid" alt={SITE_NAME} />}
                </h1>
                {errorMessage ? (
                    <div className="alert alert-danger" role="alert">
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
                <div className="checkbox mb-3">
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
                        let res: SignInResponse | null = null;
                        try {
                            res = await authenticate(email, password);
                            authContext.handleSignIn(res.session, remember);
                        } catch {
                            setErrorMessage('Invalid username or password');
                            setLoading(false);
                        }
                    }}
                    className="btn btn-lg btn-primary btn-block"
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
