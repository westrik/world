import { h, render } from 'preact';
import Router, { Route } from 'preact-router';

import UserList from '~admin/UserList';
import { AdminAuthedRoute } from '~auth/AdminAuthedRoute';
import { AuthProvider } from '~auth/AuthContext';
import { AuthedRoute } from '~auth/AuthedRoute';
import SignInForm from '~auth/SignInForm';
import ErrorScreen from '~components/ErrorScreen';
import EventLog from '~event-log/EventLog';
import NoteEditor from '~/notes/NoteEditor';
import NoteList from '~notes/NoteList';
import TaskList from '~tasks/TaskList';
import { TestTemplate } from '~components/TestTemplate';
import { SideBySide } from '~components/SideBySide';
import Note from '~notes/Note';

export default function App(): h.JSX.Element {
    return (
        <AuthProvider>
            <Router>
                <Route path="/login" component={SignInForm} />
                <AuthedRoute path="/" component={EventLog} />
                <AuthedRoute path="/tasks" component={TaskList} />
                <AuthedRoute path="/notes" component={NoteList} />
                <AuthedRoute path="/notes/new" component={Note} />
                <AuthedRoute path="/notes/:strippedApiId" component={Note} />
                <AdminAuthedRoute path="/users" component={UserList} />
                <Route default component={ErrorScreen} />

                <Route path="/side-by-side" component={SideBySide} />
                <Route path="/test-template" component={TestTemplate} />
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
