import { useMachine } from '@xstate/react';
import 'babel-polyfill';
import * as React from 'react';
import { render } from 'react-dom';
import { SITE_PROPS } from './config';
import Dashboard from './Dashboard';
import { appMachine, SESSION_KEY } from './machines/App';
import SignInForm, { Session } from './SignInForm';

const App: React.FC = () => {
  const [current, send] = useMachine(appMachine);
  if (current.matches('loading')) {
    return null;
  } else if (current.matches('signedIn')) {
    return (
      <Dashboard
        {...SITE_PROPS}
        apiToken={current.context.session!.token} // TODO: access from global store
        onSignOut={(): void => {
          send('SIGN_OUT');
        }}
      />
    );
  } else {
    return (
      <SignInForm
        {...SITE_PROPS}
        onSignIn={(persistLogin: boolean, session: Session): void => {
          (persistLogin ? localStorage : sessionStorage).setItem(
            SESSION_KEY,
            JSON.stringify(session)
          );
          send('SIGN_IN');
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
