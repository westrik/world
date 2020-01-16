import { h } from 'preact';
import Container from '../components/Container';

function NotesListing(): h.JSX.Element {
    return (
        <Container>
            <div className="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom">
                <h1 className="h2">Notes</h1>
            </div>
        </Container>
    );
}

export default NotesListing;
