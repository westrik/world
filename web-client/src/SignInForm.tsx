import * as React from 'react';
import { useState } from 'react';

import { API_HOST } from './config';
// @ts-ignore
import logo from './static/img/logo.png';
import './style/SignInForm.scss';

interface Props {
  siteName: string;
  onSignIn: (persistLogin: boolean, session: Session) => void;
}

async function authenticate(
  emailAddress: string,
  password: string
): Promise<SignInResponse> {
  const response = await fetch(`${API_HOST}/sign-in`, {
    body: JSON.stringify({ email_address: emailAddress, password }),
    // cache: 'no-cache', // *default, no-cache, reload, force-cache, only-if-cached
    // credentials: 'same-origin', // include, *same-origin, omit
    headers: {
      'Content-Type': 'application/json',
    },
    method: 'POST', // *GET, POST, PUT, DELETE, etc.
    mode: 'cors', // no-cors, *cors, same-origin
    // redirect: 'follow', // manual, *follow, error
    // referrerPolicy: 'no-referrer', // no-referrer, *client
  });
  return await response.json();
}

export interface User {
  email_address: string;
  full_name: string;
}

export interface Session {
  token: string;
  expires_at: string;
}

interface SignInResponse {
  user: User;
  session: Session;
}

const SignInForm: React.FC<Props> = props => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [remember, setRemember] = useState(false);
  const [isLoading, setLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState('');

  return (
    <div className="form-container text-center">
      <form className="form-signin">
        <h1
          className={`h3 font-weight-normal ${!errorMessage ? 'mb-3' : null}`}
        >
          {<img src={logo} className="mb-3 img-fluid" alt={props.siteName} />}
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
          onChange={(e: React.ChangeEvent<HTMLInputElement>): void =>
            setEmail(e.target.value)
          }
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
          onChange={(e: React.ChangeEvent<HTMLInputElement>): void =>
            setPassword(e.target.value)
          }
        />
        <div className="checkbox mb-3">
          <label>
            <input
              type="checkbox"
              value="remember-me"
              onClick={(e: React.MouseEvent<HTMLInputElement>): void =>
                setRemember((e.target as HTMLInputElement).checked)
              }
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
              props.onSignIn(remember, res.session);
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
};

export default SignInForm;
