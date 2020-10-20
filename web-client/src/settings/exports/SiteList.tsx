import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import LoadingSpinner from '~components/LoadingSpinner';
import ListContainer from '~components/layout/ListContainer';
import { Site } from '~models/Site';
import SiteListItem from '~settings/exports/SiteListItem';

import listSites from './listSites';

export default function SiteList(): h.JSX.Element {
    const [sites, setSites] = useState<Array<Site> | null>(null);
    const authContext = useContext(Auth);

    // TODO: refactor into custom hook
    useEffect(() => {
        if (!sites) {
            listSites(authContext, (sites) => {
                setSites(sites ?? []);
            });
        }
    });

    return sites ? (
        <ListContainer className="sites">
            {sites.map((site, key) => (
                <SiteListItem site={site} key={key} />
            ))}
        </ListContainer>
    ) : (
        <LoadingSpinner />
    );
}
