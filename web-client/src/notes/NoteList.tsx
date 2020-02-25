import { h } from 'preact';

import Container from '~components/Container';
import Header from '~components/Header';

function NoteList(): h.JSX.Element {
    return (
        <Container>
            <Header title="notes" />
        </Container>
    );
}

export default NoteList;
