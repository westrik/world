import 'babel-polyfill';
import * as React from 'react';
import { useEffect, useState } from 'react';
import { render } from 'react-dom';
import Dashboard from './Dashboard';
// eslint-disable-next-line no-unused-vars
import SignInForm, { ISession as Session, IUser as User } from './SignInForm';

const SITE_PROPS = { siteName: 'westrikworld' };
const TOKEN_KEY = 'access_token';
const EXPIRATION_KEY = 'access_expiration';

const App: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [loggedIn, setLoggedIn] = useState(false);
  const [bearerToken, setBearerToken] = useState(
    sessionStorage.getItem(TOKEN_KEY) || localStorage.getItem(TOKEN_KEY)
  );
  useEffect(() => {
    if (loading) {
      setLoading(false);
    }
    if (!loggedIn && bearerToken) {
      setLoggedIn(true);
    }
  });
  if (loading) {
    return null;
  } else if (loggedIn) {
    return (
      <Dashboard
        {...SITE_PROPS}
        onSignOut={() => {
          setLoggedIn(false);
          setBearerToken('');
          sessionStorage.clear();
          localStorage.clear();
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
          const storage = persistLogin ? localStorage : sessionStorage;
          storage.setItem(TOKEN_KEY, session.token);
          storage.setItem(EXPIRATION_KEY, session.expires_at);
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
