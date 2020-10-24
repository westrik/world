import { h } from 'preact';

import { fail } from '~utils/asserts';

import BookmarkIcon from '../static/img/icons/__generated/BookmarkIcon';
import ChevronCompactLeftIcon from '../static/img/icons/__generated/ChevronCompactLeftIcon';
import ChevronCompactRightIcon from '../static/img/icons/__generated/ChevronCompactRightIcon';
import ChevronDownIcon from '../static/img/icons/__generated/ChevronDownIcon';
import ChevronUpIcon from '../static/img/icons/__generated/ChevronUpIcon';
import CollectionIcon from '../static/img/icons/__generated/CollectionIcon';
import Diagram3Icon from '../static/img/icons/__generated/Diagram3Icon';
import ExclamationIcon from '../static/img/icons/__generated/ExclamationIcon';
import FilterIcon from '../static/img/icons/__generated/FilterIcon';
import GearIcon from '../static/img/icons/__generated/GearIcon';
import HouseIcon from '../static/img/icons/__generated/HouseIcon';
import KanbanIcon from '../static/img/icons/__generated/KanbanIcon';
import MarkdownIcon from '../static/img/icons/__generated/MarkdownIcon';
import QuestionIcon from '../static/img/icons/__generated/QuestionIcon';
import RssIcon from '../static/img/icons/__generated/RssIcon';
import SearchIcon from '../static/img/icons/__generated/SearchIcon';
import ServerIcon from '../static/img/icons/__generated/ServerIcon';
import ShareIcon from '../static/img/icons/__generated/ShareIcon';
import ShieldCheckIcon from '../static/img/icons/__generated/ShieldCheckIcon';
import ShieldXIcon from '../static/img/icons/__generated/ShieldXIcon';
import SquareCheckIcon from '../static/img/icons/__generated/SquareCheckIcon';
import SquareIcon from '../static/img/icons/__generated/SquareIcon';
import SquareSlashIcon from '../static/img/icons/__generated/SquareSlashIcon';
import TagIcon from '../static/img/icons/__generated/TagIcon';
import TagsIcon from '../static/img/icons/__generated/TagsIcon';
import TerminalIcon from '../static/img/icons/__generated/TerminalIcon';

export enum IconType {
    BOOKMARK = 'bookmark',
    CHEVRON_COMPACT_LEFT = 'chevron-compact-left',
    CHEVRON_COMPACT_RIGHT = 'chevron-compact-right',
    CHEVRON_DOWN = 'chevron-down',
    CHEVRON_UP = 'chevron-up',
    COLLECTION = 'collection',
    DIAGRAM = 'diagram-3',
    EXCLAMATION = 'exclamation',
    FILTER = 'filter',
    GEAR = 'gear', // 'gear-connected',
    HOME = 'house',
    MARKDOWN = 'markdown',
    QUESTION = 'question',
    RSS = 'rss',
    SEARCH = 'search',
    SERVER = 'server',
    SHARE = 'share',
    SHIELD_CHECK = 'shield-check',
    SHIELD_X = 'shield-x',
    SQUARE = 'square',
    SQUARE_CHECK = 'square-check',
    SQUARE_SLASH = 'square-slash',
    TAG = 'tag',
    TAGS = 'tags',
    TASKS = 'kanban',
    TERMINAL = 'terminal',
}

interface IconProps {
    iconType: IconType;
}

export default function Icon(props: IconProps): h.JSX.Element {
    switch (props.iconType) {
        case IconType.BOOKMARK:
            return <BookmarkIcon />;
        case IconType.CHEVRON_COMPACT_LEFT:
            return <ChevronCompactLeftIcon />;
        case IconType.CHEVRON_COMPACT_RIGHT:
            return <ChevronCompactRightIcon />;
        case IconType.CHEVRON_DOWN:
            return <ChevronDownIcon />;
        case IconType.CHEVRON_UP:
            return <ChevronUpIcon />;
        case IconType.COLLECTION:
            return <CollectionIcon />;
        case IconType.DIAGRAM:
            return <Diagram3Icon />;
        case IconType.EXCLAMATION:
            return <ExclamationIcon />;
        case IconType.FILTER:
            return <FilterIcon />;
        case IconType.GEAR:
            return <GearIcon />;
        case IconType.HOME:
            return <HouseIcon />;
        case IconType.MARKDOWN:
            return <MarkdownIcon />;
        case IconType.QUESTION:
            return <QuestionIcon />;
        case IconType.RSS:
            return <RssIcon />;
        case IconType.SEARCH:
            return <SearchIcon />;
        case IconType.SERVER:
            return <ServerIcon />;
        case IconType.SHARE:
            return <ShareIcon />;
        case IconType.SHIELD_CHECK:
            return <ShieldCheckIcon />;
        case IconType.SHIELD_X:
            return <ShieldXIcon />;
        case IconType.SQUARE:
            return <SquareIcon />;
        case IconType.SQUARE_CHECK:
            return <SquareCheckIcon />;
        case IconType.SQUARE_SLASH:
            return <SquareSlashIcon />;
        case IconType.TAG:
            return <TagIcon />;
        case IconType.TAGS:
            return <TagsIcon />;
        case IconType.TASKS:
            return <KanbanIcon />;
        case IconType.TERMINAL:
            return <TerminalIcon />;
        default:
            fail('Icon type not recognized');
    }
}
