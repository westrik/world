import { h } from 'preact';

import Sheet from '~components/layout/Sheet';
import PageContainer from '~components/layout/PageContainer';

interface ErrorScreenProps {
    errorDescriptionText?: string;
}

export default function ErrorScreen(props: ErrorScreenProps): h.JSX.Element {
    return (
        <PageContainer>
            <Sheet>
                <h1>{props.errorDescriptionText || '404'}</h1>
                <p>
                    <a href="/">go home</a>
                </p>
            </Sheet>
        </PageContainer>
    );
}
