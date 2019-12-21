import * as React from 'react';
import {render} from 'react-dom';
import Dashboard from './Dashboard';

const App: React.SFC = () => <Dashboard />;

render(<App />, document.getElementById('root'));

// @ts-ignore
if (module.hot) {
  // @ts-ignore
  module.hot.accept();
}
