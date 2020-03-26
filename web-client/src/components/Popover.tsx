import { h } from 'preact';
import { Manager, Popper, Reference } from 'react-popper';
import { ReactText } from 'react';

interface Props {
    children?: h.JSX.Element | Array<h.JSX.Element>;
}

type Styles = { [key: string]: ReactText };

export default function Popover(props: Props) {
    return (
        <Manager>
            <div>
                <Reference>
                    {({ ref }) => (
                        <button type="button" ref={ref!}>
                            Reference element
                        </button>
                    )}
                </Reference>
            </div>
            {/* TODO-OQ: not sure if the div wrappers are needed */}
            <div>
                <Popper placement="top-end">
                    {({ ref, style, placement, arrowProps }) => (
                        <div ref={ref!} style={style as Styles} data-placement={placement}>
                            Popper element
                            <div ref={arrowProps.ref!} style={arrowProps.style as Styles} />
                        </div>
                    )}
                </Popper>
            </div>
        </Manager>
    );
}
