import * as React from 'react';
import { render } from 'react-dom';
import Dashboard from './Dashboard.tsx';

const App: React.FC = Dashboard;

render(<App />, document.getElementById('root'));

// @ts-ignore
if (module.hot) {
  // @ts-ignore
  module.hot.accept();
}
