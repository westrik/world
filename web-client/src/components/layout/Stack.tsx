import { h } from 'preact';

export enum StackOrientation {
    HORIZONTAL = 'horizontal',
    VERTICAL = 'vertical',
}

interface StackProps {
    orientation: StackOrientation;
    children: Array<h.JSX.Element>;
}

export default function Stack(props: StackProps): h.JSX.Element {
    return <div className={`${props.orientation}-stack`}>{props.children}</div>;
}
