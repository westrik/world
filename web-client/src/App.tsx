import { h, render } from 'preact';
import Router, { Route, RouterOnChangeArgs } from 'preact-router';

import UserList from '~admin/UserList';
import { AdminAuthedRoute } from '~auth/AdminAuthedRoute';
import { AuthProvider } from '~auth/AuthContext';
import { AuthedRoute } from '~auth/AuthedRoute';
import SignInForm from '~auth/SignInForm';
import BranchList from '~branches/BranchList';
import ComponentLibraryPreview from '~components/previews/ComponentLibraryPreview';
import ErrorScreen from '~components/ErrorScreen';
import Dashboard from '~dashboard/Dashboard';
import MediaItemList from '~media/MediaItemList';
import Note from '~notes/Note';
import NoteList from '~notes/NoteList';
import SettingsPage from '~settings/SettingsPage';
import TaskList from '~tasks/TaskList';

async function handleRouteChange(event: RouterOnChangeArgs): Promise<void> {
    // TODO: update context to let us decide which tab is active
    console.log(event.url);
}

export default function App(): h.JSX.Element {
    return (
        <AuthProvider>
            <Router onChange={handleRouteChange}>
                <Route path="/login" component={SignInForm} />
                <Route path="/component-library" component={ComponentLibraryPreview} />
                <AuthedRoute path="/" component={Dashboard} />
                <AuthedRoute path="/tasks" component={TaskList} />
                <AuthedRoute path="/notes" component={NoteList} />
                <AuthedRoute path="/notes/new" component={Note} />
                <AuthedRoute path="/notes/:strippedApiId" component={Note} />
                <AuthedRoute path="/media" component={MediaItemList} />
                <AuthedRoute path="/branches" component={BranchList} />
                <AuthedRoute path="/settings" component={SettingsPage} />
                <AdminAuthedRoute path="/users" component={UserList} />
                <Route default component={ErrorScreen} />
            </Router>
        </AuthProvider>
    );
}

render(<App />, document.getElementById('root')!);

if (module.hot) {
    module.hot.accept();
}
