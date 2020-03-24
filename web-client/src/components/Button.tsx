import { h } from 'preact';
import noop from '~utils/noop';

export enum ButtonSize {
    XSMALL = 'xs',
    SMALL = 'sm',
    MEDIUM = 'md',
    LARGE = 'lg',
}

interface Props {
    title: string;
    size?: ButtonSize;
    onClick?: (ev: Event) => void;
}

export default function Button(props: Props): h.JSX.Element {
    return (
        <button className={`btn ${props.size || 'md'}`} onClick={props.onClick ? props.onClick : noop}>
            {props.title}
        </button>
    );
}
