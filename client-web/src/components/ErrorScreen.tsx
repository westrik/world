import { h } from 'preact';

interface Props {
    error?: string;
}

export default function ErrorScreen(props: Props): h.JSX.Element {
    return (
        <div className="d-flex vh-100 align-items-center">
            <div className="jumbotron container" style="max-width: 500px">
                <h1 className="display-4">{props.error || '404'}</h1>
                <p className="lead">
                    <a href="/">go home</a>
                </p>
            </div>
        </div>
    );
}
