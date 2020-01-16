import Router, { Route } from 'preact-router';
import { h, render } from 'preact';
import Dashboard from './Dashboard';
import SignInForm from './SignInForm';
import { AuthProvider } from './contexts/Auth';

function App(): h.JSX.Element {
    return (
        <AuthProvider>
            <Router>
                <Route path="/login" default component={SignInForm} />
                <Route path="/" component={Dashboard} />
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
