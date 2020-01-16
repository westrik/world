import Router, { Route } from 'preact-router';
import { h, render } from 'preact';
import SignInForm from './auth/SignInForm';
import { AuthProvider } from './auth/AuthContext';
import { AuthedRoute } from './auth/AuthedRoute';
import TasksListing from './tasks/TasksListing';
import NotesListing from './notes/NotesListing';
import DocsListing from './docs/DocsListing';

function App(): h.JSX.Element {
    return (
        <AuthProvider>
            <Router>
                <Route path="/login" component={SignInForm} />
                {/* TODO: redirect / to /tasks and change below */}
                <AuthedRoute path="/" component={TasksListing} />
                <AuthedRoute path="/notes" component={NotesListing} />
                <AuthedRoute path="/docs" component={DocsListing} />
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
