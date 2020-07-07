import { h, render } from 'preact';
import Router, { Route } from 'preact-router';

import UserList from '~admin/UserList';
import { AdminAuthedRoute } from '~auth/AdminAuthedRoute';
import { AuthProvider } from '~auth/AuthContext';
import { AuthedRoute } from '~auth/AuthedRoute';
import SignInForm from '~auth/SignInForm';
import BranchList from '~branches/BranchList';
import ComponentLibraryPreview from '~components/previews/ComponentLibraryPreview';
import ErrorScreen from '~components/ErrorScreen';
import Dashboard from '~dashboard/Dashboard';
import LibraryItemList from '~library/LibraryItemList';
import Note from '~notes/Note';
import NoteList from '~notes/NoteList';
import TaskList from '~tasks/TaskList';

export default function App(): h.JSX.Element {
    return (
        <AuthProvider>
            <Router>
                <Route path="/login" component={SignInForm} />
                <Route path="/component-library" component={ComponentLibraryPreview} />
                <AuthedRoute path="/" component={Dashboard} />
                <AuthedRoute path="/tasks" component={TaskList} />
                <AuthedRoute path="/notes" component={NoteList} />
                <AuthedRoute path="/notes/new" component={Note} />
                <AuthedRoute path="/notes/:strippedApiId" component={Note} />
                <AuthedRoute path="/library" component={LibraryItemList} />
                <AuthedRoute path="/branches" component={BranchList} />
                <AdminAuthedRoute path="/users" component={UserList} />
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
