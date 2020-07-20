import { h } from 'preact';
import { Manager, Popper, Reference } from 'react-popper';
import { ReactText } from 'react';
import { ButtonProps } from '~components/inputs/Button';
import noop from '~utils/noop';

interface Props {
    children?: h.JSX.Element | Array<h.JSX.Element>;
}

type Styles = { [key: string]: ReactText };

export function PopoverButton(props: ButtonProps): h.JSX.Element {
    return (
        <Reference>
            {({ ref }): h.JSX.Element => (
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

export default function Popover(props: Props): h.JSX.Element {
    return (
        <Manager>
            <PopoverButton title="lol button" />
            <Popper placement="top-end">
                {({ ref, style, placement, arrowProps }): h.JSX.Element => (
                    <div ref={ref!} style={style as Styles} data-placement={placement}>
                        {props.children}
                        <div ref={arrowProps.ref!} style={arrowProps.style as Styles} />
                    </div>
                )}
            </Popper>
        </Manager>
    );
}
