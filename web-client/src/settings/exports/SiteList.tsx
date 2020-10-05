import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import LoadingSpinner from '~components/LoadingSpinner';
import ListContainer from '~components/layout/ListContainer';
import { Site } from '~models/Site';
import { stripPrefixFromId } from '~utils/identifier';

import listSites from './listSites';

export default function SiteList(): h.JSX.Element {
    const [sites, setSites] = useState<Array<Site> | null>(null);
    const authContext = useContext(Auth);

    // TODO: refactor into custom hook
    useEffect(() => {
        if (!sites) {
            listSites(authContext, (sites) => {
                if (sites) {
                    setSites(sites);
                } else {
                    setSites([]);
                }
            });
        }
    });

    return sites ? (
        <ListContainer className="notes">
            {sites.map((site, key) => (
                <li draggable={true} className="site" key={key}>
                    <a href={`/sites/${stripPrefixFromId(site.id)}`}>{site.name}</a>
                </li>
            ))}
        </ListContainer>
    ) : (
        <LoadingSpinner />
    );
}
