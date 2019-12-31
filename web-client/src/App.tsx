import * as React from 'react';
import { useEffect, useState } from 'react';
import { render } from 'react-dom';
import Dashboard from './Dashboard';
import SignInForm from './SignInForm';

const SITE_PROPS = { siteName: 'westrikworld' };

const App: React.FC = () => {
  const [loggedIn, setLoggedIn] = useState(false);
  useEffect(() => {
    if (!loggedIn) {
      // TODO: load from sessionStorage / localStorage
    }
  });
  if (loggedIn) {
    return (
      <Dashboard
        {...SITE_PROPS}
        onSignOut={() => {
          setLoggedIn(false);
          // TODO: clear sessionStorage & localStorage
        }}
      />
    );
  } else {
    return (
      <SignInForm
        {...SITE_PROPS}
        onSignIn={(persistLogin: boolean) => {
          setLoggedIn(true);
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
