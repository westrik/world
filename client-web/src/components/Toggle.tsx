import { h } from 'preact';

import { randomIdentifier } from '~utils/identifier';

interface ToggleProps {
    labelText: string;
    onChange: (event: Event) => void;
}

export default function Toggle(props: ToggleProps): h.JSX.Element {
    const toggleFieldId = randomIdentifier();
    return (
        <div className="toggle-field">
            <input type="checkbox" id={toggleFieldId} tabIndex={0} onChange={(event) => props.onChange(event)} />
            <label htmlFor={toggleFieldId}>{props.labelText}</label>
        </div>
    );
}
