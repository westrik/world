import { h } from 'preact';

import { randomIdentifier } from '~utils/identifier';

export enum FieldType {
    TEXT = 'text',
    EMAIL = 'email',
    PASSWORD = 'password',
}

interface FieldProps {
    className?: string;
    labelText: string;
    placeholderText?: string;
    onChange: (event: Event) => void;
    value?: string;
    required?: boolean;
}

function InputField(props: FieldProps & { type: FieldType }): h.JSX.Element {
    const fieldId = randomIdentifier();
    return (
        <fieldset className={`${props.type}-field ${props.className}`}>
            <label htmlFor={fieldId}>{props.labelText}</label>
            <input
                type={props.type}
                id={fieldId}
                tabIndex={0}
                placeholder={props.placeholderText}
                value={props.value}
                required={props.required}
                onChange={(event): void => props.onChange(event)}
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
