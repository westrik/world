import { h } from 'preact';
import Container from '../components/Container';
import Header from '../components/Header';

function DocsListing(): h.JSX.Element {
    return (
        <Container>
            <Header title="documents" />
        </Container>
    );
}

export default DocsListing;
