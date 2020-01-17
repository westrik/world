import { h } from 'preact';

interface Props {
    title: string;
    children?: Array<h.JSX.Element>;
}

export default function Header(props: Props): h.JSX.Element {
    return (
        <div className="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom">
            <h1 className="h2">{props.title}</h1>
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
