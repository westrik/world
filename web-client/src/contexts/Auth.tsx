import { Session } from '../SignInForm';
import { createContext, h } from 'preact';

const TOKEN_KEY = 'access_token';
const EXPIRATION_KEY = 'access_expiration';

function handleSignIn(this: AuthContext, session: Session, persistSession: boolean): void {
    const storage = persistSession ? localStorage : sessionStorage;
    storage.setItem(TOKEN_KEY, session.token);
    storage.setItem(EXPIRATION_KEY, session.expires_at);
    this.authToken = session.token;
}

function handleSignOut(this: AuthContext): void {
    sessionStorage.clear();
    localStorage.clear();
    this.authToken = null;
}

interface AuthContext {
    authToken: string | null;
    handleSignIn: (session: Session, persistSession: boolean) => void;
    handleSignOut: () => void;
}

const Auth = createContext<AuthContext>({} as AuthContext);

export function AuthProvider({ children }: { children: h.JSX.Element | Array<h.JSX.Element> }): h.JSX.Element {
    return (
        <Auth.Provider
            value={{
                authToken: sessionStorage.getItem(TOKEN_KEY) || localStorage.getItem(TOKEN_KEY),
                handleSignIn,
                handleSignOut,
            }}
        >
            {children}
        </Auth.Provider>
    );
}

export default Auth;
