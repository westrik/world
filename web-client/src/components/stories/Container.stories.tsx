import { h } from 'preact';

export default { title: 'Button' };

export function withText(): h.JSX.Element {
    return <div>Hello Button</div>;
}

export function withEmoji(): h.JSX.Element {
    return (
        <div>
            <span role="img" aria-label="so cool">
                😀 😎 👍 💯
            </span>
        </div>
    );
}
