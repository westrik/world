import { h } from 'preact';

interface Props<T> {
    className?: string;
    items: Array<T>;
    selectedIndex?: number;
    renderListItem: (item: T) => h.JSX.Element | null;
    renderSelectorItem: (item: T) => string | null;
    onSelectItem: (item: T) => void;
}

interface ListItem {
    name: string;
}

export default function CollapsibleList<T extends ListItem>(props: Props<T>): h.JSX.Element {
    return (
        <div className={`collapsible-list ${props.className}`}>
            <ul>
                {props.items.map((item, key): h.JSX.Element | null => {
                    const renderedItem = props.renderListItem(item);
                    return renderedItem ? (
                        <li onClick={(): void => props.onSelectItem(item)} key={key}>
                            {renderedItem}
                        </li>
                    ) : null;
                })}
            </ul>
            <select
                aria-hidden={true}
                onChange={(ev): void => {
                    const item = props.items[(ev.target as HTMLSelectElement).selectedIndex];
                    if (!item) {
                        console.log('error: expected item');
                    } else {
                        props.onSelectItem(item);
                    }
                }}
            >
                {props.items.map((item, key): h.JSX.Element | null => {
                    const renderedItem = props.renderSelectorItem(item);
                    return renderedItem ? (
                        <option selected={key == props.selectedIndex} key={key}>
                            {renderedItem}
                        </option>
                    ) : null;
                })}
            </select>
        </div>
    );
}
