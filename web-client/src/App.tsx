import Router, { Route } from 'preact-router';
import { h, render } from 'preact';

import SignInForm from '~auth/SignInForm';
import { AuthProvider } from '~auth/AuthContext';
import { AuthedRoute } from '~auth/AuthedRoute';
import TaskList from '~tasks/TaskList';
import Stream from '~stream/Stream';
import DocumentList from '~docs/DocumentList';
import UserList from '~admin/UserList';
import { AdminAuthedRoute } from '~auth/AdminAuthedRoute';
import ErrorScreen from '~components/ErrorScreen';

export default function App(): h.JSX.Element {
    return (
        <AuthProvider>
            <Router>
                <Route path="/login" component={SignInForm} />
                <AuthedRoute path="/" component={Stream} />
                <AuthedRoute path="/tasks" component={TaskList} />
                <AuthedRoute path="/docs" component={DocumentList} />
                <AdminAuthedRoute path="/users" component={UserList} />
                <Route default component={ErrorScreen} />

                {/*<Route path="/css" component={TestTemplate} />*/}
                {/*<Route path="/editor" component={SideBySide} />*/}
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
