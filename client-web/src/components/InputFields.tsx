import { h } from 'preact';

import { randomIdentifier } from '~utils/identifier';

export enum FieldType {
    TEXT = 'text',
    EMAIL = 'email',
    PASSWORD = 'password',
}

interface FieldProps {
    labelText: string;
    placeholderText?: string;
    onChange: (event: Event) => void;
    required?: boolean;
    autoFocus?: boolean;
}

function InputField(props: FieldProps & { type: FieldType }) {
    const fieldId = randomIdentifier();
    return (
        <fieldset className={`${props.type}-field`}>
            <label htmlFor={fieldId}>{props.labelText}</label>
            <input
                type={props.type}
                id={fieldId}
                tabIndex={0}
                placeholder={props.placeholderText}
                required={props.required}
                autoFocus={props.autoFocus}
                onChange={(event) => props.onChange(event)}
            />
        </fieldset>
    );
}

export function TextField(props: FieldProps): h.JSX.Element {
    return <InputField {...props} type={FieldType.TEXT} placeholderText={props.placeholderText} />;
}

export function EmailField(props: FieldProps): h.JSX.Element {
    return <InputField {...props} type={FieldType.EMAIL} placeholderText={props.placeholderText ?? 'me@example.com'} />;
}

export function PasswordField(props: FieldProps): h.JSX.Element {
    return <InputField {...props} type={FieldType.PASSWORD} placeholderText={props.placeholderText ?? '••••••••'} />;
}
