import { h } from 'preact';
import ContentElement from '~notes/ContentElement';
import { NOTE_FIXTURE } from '~fixtures/Notes';

export default { title: 'Note' };

export function elements(): h.JSX.Element {
    return (
        <div style="max-width: 35em; margin: 0 auto;">
            {NOTE_FIXTURE.content!.elements!.map((el, key) => (
                <ContentElement element={el} key={key} />
            ))}
        </div>
    );
}
