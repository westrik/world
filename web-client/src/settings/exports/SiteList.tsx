import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import LoadingSpinner from '~components/LoadingSpinner';
import ListContainer from '~components/layout/ListContainer';
import { Site as SiteModel } from '~models/Site';

import listSites from './listSites';
import Site from './Site';

export default function SiteList(): h.JSX.Element {
    const [sites, setSites] = useState<Array<SiteModel> | null>(null);
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
                <li key={key}>
                    <Site site={site} />
                </li>
            ))}
        </ListContainer>
    ) : (
        <LoadingSpinner />
    );
}
