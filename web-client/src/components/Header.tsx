import { h } from 'preact';

interface Props {
    title: string;
    children?: Array<h.JSX.Element>;
    fixed: boolean;
    backLink?: string;
    backLinkTitle?: string;
}

export default function Header(props: Props): h.JSX.Element {
    return (
        <div
            style={
                props.fixed
                    ? 'position: fixed; z-index: 1; background: #fff; box-shadow: 0px 20px 20px 0px rgba(255,255,255,1); width: 100%; max-width: 75%'
                    : ''
            }
            className="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom"
        >
            <h1 className="h2">
                {props.backLink ? (
                    <a className="back-link" href={props.backLink}>
                        &larr; {props.backLinkTitle ? props.backLinkTitle : 'back'}
                    </a>
                ) : null}
                {props.title}
            </h1>
            {props.children ? (
                <div className="form-inline">
                    {props.children.map((child, key) => (
                        <div key={key} className="ml-2 float-left">
                            {child}
                        </div>
                    ))}
                </div>
            ) : null}
        </div>
    );
}
