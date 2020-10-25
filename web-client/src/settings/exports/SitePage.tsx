import { h } from 'preact';
// import {useContext, useEffect} from 'preact/hooks';

// import Auth from '~auth/AuthContext';
import Button, { ButtonSize } from '~components/Button';
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
                    <td>Note ID</td>
                    <td>{props.page.noteId}</td>
                </tr>
                <tr>
                    <td>Note Version ID</td>
                    <td>{props.page.noteVersionId}</td>
                </tr>
                <tr>
                    <td>Published?</td>
                    <td>{props.page.published ? 'true' : 'false'}</td>
                </tr>
            </table>
            {props.page.published ? (
                <Button title="Un-Publish" size={ButtonSize.XSMALL} />
            ) : (
                <Button title="Publish" size={ButtonSize.XSMALL} />
            )}
            {/*{props.page.published && canUpdatePage() && <Button title="Publish Updates" />}*/}
            <Button title="Delete" size={ButtonSize.XSMALL} />
        </div>
    );
}
