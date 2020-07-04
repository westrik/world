import { h } from 'preact';

interface Props {
    children: h.JSX.Element | Array<h.JSX.Element>;
    acceptFocus?: boolean;
}

export default function Tile(props: Props): h.JSX.Element {
    let tileProps = {};
    if (props.acceptFocus) {
        tileProps = { tabIndex: 0 };
    }
    return (
        <div {...tileProps} className="tile">
            {props.children}
        </div>
    );
}
