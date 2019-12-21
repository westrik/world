import * as React from 'react';
import { useState } from 'react';
import { render } from 'react-dom';
import Dashboard from './Dashboard';
import SignInForm from './SignInForm';

const App: React.FC = () => {
  const [loggedIn, setLoggedIn] = useState(false);
  if (loggedIn) {
    return <Dashboard />;
  } else {
    return <SignInForm />;
  }
};

render(<App />, document.getElementById('root'));

// @ts-ignore
if (module.hot) {
  // @ts-ignore
  module.hot.accept();
}
