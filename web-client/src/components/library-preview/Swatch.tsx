import { h } from 'preact';

interface SwatchProps {
    colorName: string;
}

export default function Swatch(props: SwatchProps): h.JSX.Element {
    return (
        <div className="color-swatch">
            <div className={`bg-color-${props.colorName}`} />
            <h6>{props.colorName}</h6>
        </div>
    );
}
