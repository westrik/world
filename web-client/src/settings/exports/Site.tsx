import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import Button, { ButtonSize } from '~components/Button';
import LoadingSpinner from '~components/LoadingSpinner';
import ListContainer from '~components/layout/ListContainer';
import { Site as SiteModel } from '~models/Site';
import { SitePage as SitePageModel } from '~models/SitePage';
import { stripPrefixFromId } from '~utils/identifier';

import listSitePages from './listSitePages';
import SitePage from './SitePage';

interface SiteProps {
    site: SiteModel;
}

export default function Site(props: SiteProps): h.JSX.Element {
    const [pages, setPages] = useState<Array<SitePageModel> | null>(null);
    const authContext = useContext(Auth);

    // TODO: refactor into custom hook
    useEffect(() => {
        if (!pages) {
            listSitePages(authContext, props.site.id, (pages) => {
                setPages(pages ?? []);
            });
        }
    });

    return (
        <div className="site">
            <a href={`/sites/${stripPrefixFromId(props.site.id)}`}>{props.site.description}</a>
            <br />
            <Button title="Configure" size={ButtonSize.XSMALL} />
            <br />
            <Button title="Create Page" size={ButtonSize.XSMALL} />

            {pages ? (
                <ListContainer className="site-pages">
                    {pages.map((page, key) => (
                        <li key={key}>
                            <SitePage page={page} />
                        </li>
                    ))}
                </ListContainer>
            ) : (
                <LoadingSpinner />
            )}
        </div>
    );
}
