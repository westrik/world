import { h } from 'preact';
import Button, { ButtonSize } from '~components/Button';
import { createPopper } from '@popperjs/core';

export default { title: 'Button' };

export function normal(): h.JSX.Element {
    return (
        <ul>
            <li>
                <Button size={ButtonSize.LARGE} title="Test Button (Large)" />
            </li>
            <li>
                <Button title="Test Button (Default)" />
            </li>
            <li>
                <Button size={ButtonSize.SMALL} title="Test Button (Small)" />
            </li>
            <li>
                <Button size={ButtonSize.XSMALL} title="Test Button (X-Small)" />
            </li>
        </ul>
    );
}

export function withPopover(): h.JSX.Element {
    const newPopper = (ev: Event): void => {
        const target = ev.target as HTMLButtonElement;
        const popper = document.querySelector('#popper');
        createPopper(target, popper as HTMLElement);
    };
    return (
        <div>
            <div style="position:absolute;top:-9999rem;left:-9999rem;" id="popper">
                this is a popover
            </div>
            <ul>
                <li>
                    <Button onClick={newPopper} size={ButtonSize.LARGE} title="large" />
                </li>
                <li>
                    <Button onClick={newPopper} title="default" />
                </li>
                <li>
                    <Button onClick={newPopper} size={ButtonSize.SMALL} title="small" />
                </li>
                <li>
                    <Button onClick={newPopper} size={ButtonSize.XSMALL} title="x-small" />
                </li>
            </ul>
        </div>
    );
}
