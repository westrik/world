import 'babel-polyfill';
import * as React from 'react';
import { useEffect, useState } from 'react';
import { render } from 'react-dom';
import Dashboard from './Dashboard';
// eslint-disable-next-line no-unused-vars
import SignInForm, { ISession as Session, IUser as User } from './SignInForm';

const SITE_PROPS = { siteName: 'westrikworld' };

const App: React.FC = () => {
  const [loggedIn, setLoggedIn] = useState(false);
  const [bearerToken, setBearerToken] = useState(''); // TODO: load from sessionStorage / localStorage
  useEffect(() => {
    if (!loggedIn && bearerToken) {
      setLoggedIn(true);
    }
  });
  if (loggedIn) {
    return (
      <Dashboard
        {...SITE_PROPS}
        onSignOut={() => {
          setLoggedIn(false);
          setBearerToken('');
          // TODO: clear sessionStorage & localStorage
        }}
      />
    );
  } else {
    return (
      <SignInForm
        {...SITE_PROPS}
        onSignIn={(persistLogin: boolean, user: User, session: Session) => {
          setLoggedIn(true);
          setBearerToken(session.token);
          // tslint:disable-next-line:no-console
          console.log(user);
          // tslint:disable-next-line:no-console
          console.log(session);
          if (persistLogin) {
            // TODO: set localStorage
          } else {
            // TODO: set sessionStorage
          }
        }}
      />
    );
  }
};

render(<App />, document.getElementById('root'));

// @ts-ignore
if (module.hot) {
  // @ts-ignore
  module.hot.accept();
}
