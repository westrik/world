import { h } from 'preact';
import AppContainer from '~components/AppContainer';
import ListContainer from '~components/ListContainer';

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
