import Router, { Route } from 'preact-router';
import { h, render } from 'preact';
import Dashboard from './Dashboard';
import SignInForm from './SignInForm';
import { AuthProvider } from './auth/AuthContext';
import { AuthedRoute } from './auth/AuthedRoute';

function App(): h.JSX.Element {
    return (
        <AuthProvider>
            <Router>
                <Route path="/login" component={SignInForm} />
                <AuthedRoute path="/" component={Dashboard} />
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
