import { h } from 'preact';
import Container from '../components/Container';
import Header from '../components/Header';

function DocumentList(): h.JSX.Element {
    return (
        <Container>
            <Header title="documents" />
        </Container>
    );
}

export default DocumentList;
