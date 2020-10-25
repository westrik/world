import { h } from 'preact';
// import {useContext, useEffect} from 'preact/hooks';

// import Auth from '~auth/AuthContext';
import { SitePage as SitePageModel } from '~models/SitePage';

interface SitePageProps {
    page: SitePageModel;
}

export default function SitePage(props: SitePageProps): h.JSX.Element {
    // const authContext = useContext(Auth);
    // TODO: refactor into custom hook
    // useEffect(() => {
    // });

    return (
        <div className="site-page">
            <table>
                <tr>
                    <td>ID</td>
                    <td>{props.page.id}</td>
                </tr>
                <tr>
                    <td>URL</td>
                    <td>{props.page.path}</td>
                </tr>
                <tr>
                    <td>Note Version ID</td>
                    <td>{props.page.noteVersionId}</td>
                </tr>
            </table>
        </div>
    );
}
