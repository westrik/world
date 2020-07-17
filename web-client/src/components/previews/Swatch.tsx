import { h } from 'preact';

interface SwatchProps {
    colorName: string;
}

export default function Swatch(props: SwatchProps): h.JSX.Element {
    return <div className={`bg-color-${props.colorName}`}>COLOR TEST</div>;
}
