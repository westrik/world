import { h } from 'preact';

import Container from '~/components/Container';
import Header from '~/components/Header';

function UserList(): h.JSX.Element {
    return (
        <Container>
            <Header title="users" />
        </Container>
    );
}

export default UserList;
