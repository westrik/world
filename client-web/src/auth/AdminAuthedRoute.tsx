import { h } from 'preact';
import { Route, RouteProps } from 'preact-router';
import { useContext, useEffect } from 'preact/hooks';

import Auth from './AuthContext';

export function AdminAuthedRoute<Props>(props: RouteProps<Props> & Partial<Props>): preact.VNode {
    const authContext = useContext(Auth);
    useEffect(() => {
        // TODO: automatically sign out if token is expired (re-use logic from AuthedRoute)
        // TODO: store separate field for admin users
        if (!authContext.authToken) {
            authContext.handleSignOut();
        }
    });
    return authContext.isLoggedIn() ? <Route {...props} /> : <Route component={(): h.JSX.Element => <div />} />;
}
