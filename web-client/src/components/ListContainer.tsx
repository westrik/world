import { h } from 'preact';

interface Props {
    className?: string;
    children: h.JSX.Element | Array<h.JSX.Element>;
}

export default function ListContainer(props: Props): h.JSX.Element {
    return <ul className={`list-container ${props.className}`}>{props.children}</ul>;
}
