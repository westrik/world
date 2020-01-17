import { h } from 'preact';
import Container from '../components/Container';
import Header from '../components/Header';

function NotesListing(): h.JSX.Element {
    return (
        <Container>
            <Header title="note-stream" />
        </Container>
    );
}

export default NotesListing;
