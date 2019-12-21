import * as React from 'react';

import './style/SignInForm.scss';

interface IProps {
  onSignIn: () => void;
}

const SignInForm: React.FC<IProps> = props => (
  <div className="form-container text-center">
    <form className="form-signin">
      <h1 className="h3 mb-3 font-weight-normal">
        Sign in to <em>Timeline</em>
      </h1>
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
      />
      <div className="checkbox mb-3">
        <label>
          <input type="checkbox" value="remember-me" /> Remember me
        </label>
      </div>
      <button
        onClick={props.onSignIn}
        className="btn btn-lg btn-primary btn-block"
        type="submit"
      >
        Sign in
      </button>
    </form>
  </div>
);

export default SignInForm;
