import { Route, route, RouteProps } from 'preact-router';
import { useContext, useEffect } from 'preact/hooks';
import Auth from './AuthContext';
import { h } from 'preact';

export function AuthedRoute<Props>(props: RouteProps<Props> & Partial<Props>): preact.VNode {
    const authContext = useContext(Auth);
    useEffect(() => {
        // TODO: automatically sign out if token is expired
        if (!authContext.authToken) {
            route('/login');
        }
    });
    return authContext.isLoggedIn() ? <Route {...props} /> : <Route component={(): h.JSX.Element => <div />} />;
}
