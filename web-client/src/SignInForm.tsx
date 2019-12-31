import * as React from 'react';
import { useState } from 'react';

import './style/SignInForm.scss';

interface IProps {
  siteName: string;
  onSignIn: (persistLogin: boolean) => void;
}

const SignInForm: React.FC<IProps> = props => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [remember, setRemember] = useState(false);
  const [isLoading, setLoading] = useState(false);

  return (
    <div className="form-container text-center">
      <form className="form-signin">
        <h1 className="h3 mb-3 font-weight-normal">{props.siteName}</h1>
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
          onClick={() => {
            setLoading(true);
            // props.onSignIn(remember);
            // tslint:disable-next-line:no-console
            console.log({ email, password, remember });
          }}
          className="btn btn-lg btn-primary btn-block"
          type="submit"
          disabled={isLoading}
        >
          {isLoading ? (
            <span
              className="spinner-grow spinner-grow-sm"
              role="status"
              aria-hidden="true"
            />
          ) : null}{' '}
          Sign in
        </button>
      </form>
    </div>
  );
};

export default SignInForm;
