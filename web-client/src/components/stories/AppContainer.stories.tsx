import { h } from 'preact';
import AppContainer from '~components/AppContainer';
import ListContainer from '~components/layout/ListContainer';

// At least one .stories.tsx file needs this import so webpack knows to load styles
import '~/style/app.scss';

export default { title: 'App Container' };

export function normal(): h.JSX.Element {
    return (
        <AppContainer>
            <ListContainer>
                <li>Hello</li>
                <li>World</li>
            </ListContainer>
        </AppContainer>
    );
}
