import { h } from 'preact';

import Icon, { IconType } from '~components/Icon';

export default { title: 'Icons' };

const ICONS: Array<IconType> = [
    IconType.BOOKMARK,
    IconType.CHEVRON_COMPACT_LEFT,
    IconType.CHEVRON_COMPACT_RIGHT,
    IconType.CHEVRON_DOWN,
    IconType.CHEVRON_UP,
    IconType.COLLECTION,
    IconType.DIAGRAM,
    IconType.EXCLAMATION,
    IconType.FILTER,
    IconType.GEAR,
    IconType.HOME,
    IconType.MARKDOWN,
    IconType.QUESTION,
    IconType.RSS,
    IconType.SEARCH,
    IconType.SERVER,
    IconType.SHARE,
    IconType.SHIELD_CHECK,
    IconType.SHIELD_X,
    IconType.SQUARE,
    IconType.SQUARE_CHECK,
    IconType.SQUARE_SLASH,
    IconType.TAG,
    IconType.TAGS,
    IconType.TASKS,
    IconType.TERMINAL,
];

export function normal(): h.JSX.Element {
    return (
        <ul>
            {ICONS.map((icon, key) => {
                return (
                    <li key={key}>
                        <Icon iconType={icon} />
                    </li>
                );
            })}
        </ul>
    );
}
