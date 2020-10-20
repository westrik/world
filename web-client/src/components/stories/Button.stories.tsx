import { h } from 'preact';
import Button, { ButtonSize, ButtonVariant } from '~components/Button';
import Popover from '~components/Popover';

export default { title: 'Button' };

export function normal(): h.JSX.Element {
    return (
        <ul className="story-container">
            <li>
                <Button variant={ButtonVariant.PRIMARY} size={ButtonSize.LARGE} title="Test Button (Large, Primary)" />
            </li>
            <li>
                <Button title="Test Button (Medium, Secondary)" />
            </li>
            <li>
                <Button variant={ButtonVariant.TERTIARY} title="Test Button (Medium, Tertiary)" />
            </li>
            <li>
                <Button size={ButtonSize.SMALL} title="Test Button (Small, Secondary)" />
            </li>
            <li>
                <Button size={ButtonSize.XSMALL} title="Test Button (X-Small, Secondary)" />
            </li>
            <li>
                <Button
                    default={true}
                    disabled={true}
                    size={ButtonSize.LARGE}
                    title="Test Button (Large, Default, Disabled)"
                />
            </li>
            <li>
                <Button default={true} title="Test Button (Medium, Default)" />
            </li>
            <li>
                <Button disabled={true} size={ButtonSize.SMALL} title="Test Button (Small, Disabled)" />
            </li>
            <li>
                <Button disabled={true} size={ButtonSize.XSMALL} title="Test Button (X-Small, Disabled)" />
            </li>
        </ul>
    );
}

export function withPopover(): h.JSX.Element {
    const showPopper = (): void => {
        console.log('TODO: show popper');
    };
    return (
        <div className="story-container">
            <div>
                <Popover key="popover" />
            </div>
            <div>
                <Button onClick={showPopper} size={ButtonSize.LARGE} title="large" />
            </div>
            <div>
                <Button onClick={showPopper} title="default" />
            </div>
            <div>
                <Button onClick={showPopper} size={ButtonSize.SMALL} title="small" />
            </div>
            <div>
                <Button onClick={showPopper} size={ButtonSize.XSMALL} title="x-small" />
            </div>
        </div>
    );
}
