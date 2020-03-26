import { h } from 'preact';

interface Props {
    className?: string;
}

export default function LoadingSpinner(props: Props): h.JSX.Element {
    return (
        <div className={`loading-spinner ${props.className}`} role="status">
            <span className="sr-only">Loading...</span>
        </div>
    );
}
