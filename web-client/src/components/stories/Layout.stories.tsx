import { h } from 'preact';

import NavSidebar from '~components/NavSidebar';

export default { title: 'Layout' };

export function normal(): h.JSX.Element {
    return (
        <div>
            <NavSidebar />
            <main />
        </div>
    );
}
