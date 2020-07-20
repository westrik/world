import { h } from 'preact';

interface SubmitButtonProps {
    text: string;
    disabled?: boolean;
    onButtonPress: (event: Event) => void;
}

export default function SubmitButton(props: SubmitButtonProps): h.JSX.Element {
    return (
        <fieldset className="submit-button">
            <button
                type="submit"
                tabIndex={0}
                disabled={props.disabled}
                onClick={(event): void => props.onButtonPress(event)}
            >
                {props.text}
            </button>
        </fieldset>
    );
}
