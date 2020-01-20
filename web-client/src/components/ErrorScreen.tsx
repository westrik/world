import { h } from 'preact';

interface Props {
    error?: string;
}

export default function ErrorScreen(props: Props): h.JSX.Element {
    return <h1>{props.error || '404'}</h1>;
}
