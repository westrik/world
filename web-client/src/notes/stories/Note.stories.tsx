import { h } from 'preact';

import PageContainer from '~components/PageContainer';
import Sheet from '~components/Sheet';
import { NOTE_FIXTURE } from '~fixtures/Notes';
import ContentElement from '~notes/ContentElement';

export default { title: 'Note' };

export function elements(): h.JSX.Element {
    return (
        <PageContainer>
            <Sheet>
                <div className="article">
                    <div className="elements">
                        {NOTE_FIXTURE.content!.elements!.map((el, key) => (
                            <ContentElement element={el} key={key} />
                        ))}
                    </div>
                </div>
            </Sheet>
        </PageContainer>
    );
}
