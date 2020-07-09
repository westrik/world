import { h } from 'preact';

import ErrorScreen from '~components/ErrorScreen';

export default { title: 'Error Screen' };

export function normal(): h.JSX.Element {
    return <ErrorScreen />;
}

export function withErrorDescription(): h.JSX.Element {
    return <ErrorScreen errorDescriptionText="Oops! You broke it." />;
}
