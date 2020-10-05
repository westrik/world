import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import LoadingSpinner from '~components/LoadingSpinner';
import ListContainer from '~components/layout/ListContainer';
import { Site } from '~models/Site';
import { SitePage } from '~models/SitePage';

import listSitePages from './listSitePages';
import { stripPrefixFromId } from '~utils/identifier';

interface SiteListItemProps {
    site: Site;
}

export default function SiteListItem(props: SiteListItemProps): h.JSX.Element {
    const [pages, setPages] = useState<Array<SitePage> | null>(null);
    const authContext = useContext(Auth);

    // TODO: refactor into custom hook
    useEffect(() => {
        if (!pages) {
            listSitePages(authContext, props.site.id, (pages) => {
                if (pages) {
                    setPages(pages);
                } else {
                    setPages([]);
                }
            });
        }
    });

    return (
        <div>
            <li className="site">
                <a href={`/sites/${stripPrefixFromId(props.site.id)}`}>{props.site.description}</a>

                {pages ? (
                    <ListContainer className="site-pages">
                        {pages.map((page, key) => (
                            <li className="site-page" key={key}>
                                {page.path}
                            </li>
                        ))}
                    </ListContainer>
                ) : (
                    <LoadingSpinner />
                )}
            </li>
        </div>
    );
}
