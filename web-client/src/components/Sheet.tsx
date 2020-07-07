import { h } from 'preact';

interface SheetProps {
    children: h.JSX.Element | Array<h.JSX.Element>;
}

export default function Sheet(props: SheetProps): h.JSX.Element {
    return <div className="sheet-container">{props.children}</div>;
}
