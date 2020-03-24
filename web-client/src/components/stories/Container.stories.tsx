import { h } from 'preact';
import Container from '~components/Container';
import ListContainer from '~components/ListContainer';

export default { title: 'Container' };

export function normal(): h.JSX.Element {
    return (
        <Container>
            <ListContainer>
                <li>Hello</li>
                <li>World</li>
            </ListContainer>
        </Container>
    );
}
