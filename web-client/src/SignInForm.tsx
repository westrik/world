import * as React from 'react';
import { useState } from 'react';

// @ts-ignore: picked up by parcel, but tsc doesn't like it
import logo from './static/img/logo.png';
import './style/SignInForm.scss';

interface IProps {
  siteName: string;
  onSignIn: (persistLogin: boolean) => void;
}

const DEV_HOST = 'http://api.westrik.world:6874';

async function authenticate(emailAddress: string, password: string) {
  const response = await fetch(`${DEV_HOST}/sign-in`, {
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

const SignInForm: React.FC<IProps> = props => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [remember, setRemember] = useState(false);
  const [isLoading, setLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState('');

  return (
    <div className="form-container text-center">
      <form className="form-signin">
        <h1 className="h3 mb-3 font-weight-normal">
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
          onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
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
          onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
            setPassword(e.target.value)
          }
        />
        <div className="checkbox mb-3">
          <label>
            <input
              type="checkbox"
              value="remember-me"
              onClick={(e: React.MouseEvent<HTMLInputElement>) =>
                // @ts-ignore: not sure the types are correct?
                setRemember(e.target.checked)
              }
            />{' '}
            Remember me
          </label>
        </div>
        <button
          onClick={async event => {
            event.preventDefault();

            setLoading(true);
            let res;
            try {
              res = await authenticate(email, password);
            } catch {
              setErrorMessage('Invalid username or password');
            }
            // tslint:disable-next-line:no-console
            console.log(res);
            setLoading(false);
            // props.onSignIn(remember);
            // tslint:disable-next-line:no-console
            console.log({remember});
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
