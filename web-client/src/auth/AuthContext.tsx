import { createContext, h } from 'preact';
import { route } from 'preact-router';

import { Session } from '~models/Session';

const TOKEN_KEY = 'access_token';
const EXPIRATION_KEY = 'access_expiration';

interface AuthContext {
    authToken: string | null;
    handleSignIn: (session: Session, persistSession: boolean) => void;
    handleSignOut: () => void;
    isLoggedIn: () => boolean;
}

const Auth = createContext<AuthContext>({} as AuthContext);

function handleSignIn(this: AuthContext, session: Session, persistSession: boolean): void {
    const storage = persistSession ? localStorage : sessionStorage;
    storage.setItem(TOKEN_KEY, session.token);
    storage.setItem(EXPIRATION_KEY, session.expires_at);
    this.authToken = session.token;
    route('/');
}

function handleSignOut(this: AuthContext): void {
    sessionStorage.clear();
    localStorage.clear();
    this.authToken = null;
    route('/login');
}

function isLoggedIn(this: AuthContext): boolean {
    return Boolean(this.authToken);
}

export function AuthProvider({ children }: { children: h.JSX.Element | Array<h.JSX.Element> }): h.JSX.Element {
    return (
        <Auth.Provider
            value={{
                authToken: sessionStorage.getItem(TOKEN_KEY) || localStorage.getItem(TOKEN_KEY),
                handleSignIn,
                handleSignOut,
                isLoggedIn,
            }}
        >
            {children}
        </Auth.Provider>
    );
}

export default Auth;
