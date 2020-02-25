import { h } from 'preact';

export default function ListContainer({ children }: { children: Array<h.JSX.Element> }): h.JSX.Element {
    return <ul style="list-style: none; padding: 0; padding-bottom: 4em;">{children}</ul>;
}
