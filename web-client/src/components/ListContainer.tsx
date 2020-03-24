import { h } from 'preact';

export default function ListContainer({ children }: { children: Array<h.JSX.Element> }): h.JSX.Element {
    return <ul className="list-container">{children}</ul>;
}
