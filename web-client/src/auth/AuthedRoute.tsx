import { h } from 'preact';
import { useContext, useEffect } from 'preact/hooks';
import { Route, RouteProps } from 'preact-router';

import Auth from './AuthContext';

export function AuthedRoute<Props>(props: RouteProps<Props> & Partial<Props>): preact.VNode {
    const authContext = useContext(Auth);
    useEffect(() => {
        // TODO: automatically sign out if token is expired
        if (!authContext.authToken) {
            authContext.handleSignOut();
        }
    });
    return authContext.isLoggedIn() ? <Route {...props} /> : <Route component={(): h.JSX.Element => <div />} />;
}
