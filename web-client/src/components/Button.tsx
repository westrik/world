import { h } from 'preact';
import noop from '~utils/noop';

export enum ButtonSize {
    XSMALL = 'xs',
    SMALL = 'sm',
    MEDIUM = 'md',
    LARGE = 'lg',
}

export enum ButtonVariant {
    PRIMARY = 'primary',
    SECONDARY = 'secondary',
    TERTIARY = 'tertiary',
}

export interface ButtonProps {
    title: string;
    size?: ButtonSize;
    variant?: ButtonVariant;
    disabled?: boolean;
    default?: boolean;
    onClick?: (ev: Event) => void;
}

export default function Button(props: ButtonProps): h.JSX.Element {
    return (
        <button
            disabled={props.disabled}
            className={`button-${props.variant || ButtonVariant.SECONDARY} ${props.size || ButtonSize.MEDIUM} ${
                props.default ? 'default' : ''
            }`}
            onClick={props.onClick ? props.onClick : noop}
        >
            {props.title}
        </button>
    );
}
