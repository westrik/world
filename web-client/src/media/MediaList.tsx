import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import Container from '~components/Container';
import ListContainer from '~components/ListContainer';
import LoadingSpinner from '~components/LoadingSpinner';
import Header from '~components/Header';
import { Note } from '~models/Note';

import listMedia from './listMedia';
import MediaTile from './MediaTile';

function MediaList(): h.JSX.Element {
    const [mediaSummaries, setMediaSummaries] = useState<Array<Note> | null>(null);
    const authContext = useContext(Auth);

    useEffect(() => {
        if (!mediaSummaries) {
            listMedia(authContext, (notes) => {
                if (notes) {
                    setMediaSummaries(notes);
                } else {
                    setMediaSummaries([]);
                }
            });
        }
    });

    return (
        <Container>
            <Header title="notes" fixed={true} />
            {mediaSummaries ? (
                <ListContainer>
                    {mediaSummaries.map((note, key) => (
                        <li className="note-item" key={key}>
                            <MediaTile note={note} />
                        </li>
                    ))}
                </ListContainer>
            ) : (
                <LoadingSpinner />
            )}
        </Container>
    );
}

export default MediaList;
