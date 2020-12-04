import { h } from 'preact';

import AppContainer from '~components/AppContainer';
import { TextField } from '~components/InputFields';

export default function Dashboard(): h.JSX.Element {
    return (
        <AppContainer>
            <TextField
                labelText="Search"
                onChange={() => {
                    console.log('updated');
                }}
            />
            {/* TODO: task summary */}
            {/* TODO: recent notes */}
            {/* TODO: IoT stats */}
        </AppContainer>
    );
}
