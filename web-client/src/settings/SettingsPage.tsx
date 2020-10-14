import { h } from 'preact';

import AppContainer from '~components/AppContainer';
import SiteList from '~settings/exports/SiteList';

export default function SettingsPage(): h.JSX.Element {
    return (
        <AppContainer>
            <SiteList />
        </AppContainer>
    );
}
