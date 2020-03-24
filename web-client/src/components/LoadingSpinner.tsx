import { h } from 'preact';

export default function LoadingSpinner(): h.JSX.Element {
    return (
        <div className="loading-spinner" role="status">
            <span className="sr-only">Loading...</span>
        </div>
    );
}
