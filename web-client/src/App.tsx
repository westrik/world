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

const API_HOSTS = {
  local: 'http://api.westrik.world:6874',
  production: 'https://api.westrikworld.com',
  staging: 'https://api.staging.westrikworld.com',
};
const env = process.env.NODE_ENV;
export const API_HOST =
  env === 'staging'
    ? API_HOSTS.staging
    : env === 'production'
      ? API_HOSTS.production
      : API_HOSTS.local;

const App: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [bearerToken, setBearerToken] = useState(
    sessionStorage.getItem(TOKEN_KEY) || localStorage.getItem(TOKEN_KEY)
  );
  useEffect(() => {
    if (loading) {
      setLoading(false);
    }
  });
  if (loading) {
    return null;
  } else if (bearerToken) {
    return (
      <Dashboard
        {...SITE_PROPS}
        apiToken={bearerToken}
        onSignOut={() => {
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
