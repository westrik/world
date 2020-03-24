import { h } from 'preact';

interface Props {
    children: h.JSX.Element | Array<h.JSX.Element>;
}

export default function Tile(props: Props): h.JSX.Element {
    return <div className="tile">{props.children}</div>;
}
