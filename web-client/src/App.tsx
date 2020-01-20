import Router, { Route } from 'preact-router';
import { h, render } from 'preact';
import SignInForm from './auth/SignInForm';
import { AuthProvider } from './auth/AuthContext';
import { AuthedRoute } from './auth/AuthedRoute';
import TasksListing from './tasks/TasksListing';
import Stream from './stream/Stream';
import DocsListing from './docs/DocsListing';
import UsersListing from './admin/UsersListing';
import { AdminAuthedRoute } from './auth/AdminAuthedRoute';
import ErrorScreen from './components/ErrorScreen';

function App(): h.JSX.Element {
    return (
        <AuthProvider>
            <Router>
                <Route path="/login" component={SignInForm} />
                <AuthedRoute path="/" component={Stream} />
                <AuthedRoute path="/tasks" component={TasksListing} />
                <AuthedRoute path="/docs" component={DocsListing} />
                <AdminAuthedRoute path="/users" component={UsersListing} />
                <Route default component={ErrorScreen} />
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
