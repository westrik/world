import { h, RefObject } from 'preact';
import { Manager, Popper, Reference } from 'react-popper';
import { ReactText } from 'react';
import PopperButton, { ButtonProps } from '~components/Button';
import noop from '~utils/noop';

interface Props {
    children?: h.JSX.Element | Array<h.JSX.Element>;
}

type Styles = { [key: string]: ReactText };

export function PopoverButton(props: ButtonProps): h.JSX.Element {
    return (
        <Reference>
            {({ ref }) => (
                <button
                    ref={ref!}
                    disabled={props.disabled}
                    className={`button ${props.size || 'md'} ${props.default ? 'default' : ''}`}
                    onClick={props.onClick ? props.onClick : noop}
                >
                    {props.title}
                </button>
            )}
        </Reference>
    );
}

export default function Popover(props: Props) {
    return (
        <Manager>
            <PopoverButton title="lol button" />
            <Popper placement="top-end">
                {({ ref, style, placement, arrowProps }) => (
                    <div ref={ref!} style={style as Styles} data-placement={placement}>
                        Popper element
                        <div ref={arrowProps.ref!} style={arrowProps.style as Styles} />
                    </div>
                )}
            </Popper>
        </Manager>
    );
}
