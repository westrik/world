import { h } from 'preact';
import noop from '~utils/noop';

export enum ButtonSize {
    XSMALL = 'xs',
    SMALL = 'sm',
    MEDIUM = 'md',
    LARGE = 'lg',
}

export interface ButtonProps {
    title: string;
    size?: ButtonSize;
    disabled?: boolean;
    default?: boolean;
    onClick?: (ev: Event) => void;
}

export default function Button(props: ButtonProps): h.JSX.Element {
    return (
        <button
            disabled={props.disabled}
            className={`button ${props.size || 'md'} ${props.default ? 'default' : ''}`}
            onClick={props.onClick ? props.onClick : noop}
        >
            {props.title}
        </button>
    );
}
