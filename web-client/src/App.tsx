import Router, { Route } from 'preact-router';
import { h, render } from 'preact';
import Dashboard from './Dashboard';
import SignInForm from './SignInForm';
import { AuthProvider } from './contexts/Auth';

export const SITE_NAME = 'westrikworld';
const API_HOSTS = {
    local: 'http://api.westrik.world:6874',
    production: 'https://api.westrikworld.com',
    staging: 'https://api.staging.westrikworld.com',
};
const env = process.env.NODE_ENV;
export const API_HOST =
    env === 'staging' ? API_HOSTS.staging : env === 'production' ? API_HOSTS.production : API_HOSTS.local;

// TODO:
//  - set browser history

function App(): h.JSX.Element | null {
    return (
        <AuthProvider>
            <Router>
                <Route path="/login" default component={SignInForm} />
                <Route path="/tasks" component={Dashboard} />
            </Router>
        </AuthProvider>
    );
}

render(<App />, document.getElementById('root')!);

// @ts-ignore
if (module.hot) {
    // @ts-ignore
    module.hot.accept();
}
