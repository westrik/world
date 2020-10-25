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
    let iconComponent: h.JSX.Element;
    switch (props.iconType) {
        case IconType.BOOKMARK:
            iconComponent = <BookmarkIcon />;
            break;
        case IconType.CHEVRON_COMPACT_LEFT:
            iconComponent = <ChevronCompactLeftIcon />;
            break;
        case IconType.CHEVRON_COMPACT_RIGHT:
            iconComponent = <ChevronCompactRightIcon />;
            break;
        case IconType.CHEVRON_DOWN:
            iconComponent = <ChevronDownIcon />;
            break;
        case IconType.CHEVRON_UP:
            iconComponent = <ChevronUpIcon />;
            break;
        case IconType.COLLECTION:
            iconComponent = <CollectionIcon />;
            break;
        case IconType.DIAGRAM:
            iconComponent = <Diagram3Icon />;
            break;
        case IconType.EXCLAMATION:
            iconComponent = <ExclamationIcon />;
            break;
        case IconType.FILTER:
            iconComponent = <FilterIcon />;
            break;
        case IconType.GEAR:
            iconComponent = <GearIcon />;
            break;
        case IconType.HOME:
            iconComponent = <HouseIcon />;
            break;
        case IconType.MARKDOWN:
            iconComponent = <MarkdownIcon />;
            break;
        case IconType.QUESTION:
            iconComponent = <QuestionIcon />;
            break;
        case IconType.RSS:
            iconComponent = <RssIcon />;
            break;
        case IconType.SEARCH:
            iconComponent = <SearchIcon />;
            break;
        case IconType.SERVER:
            iconComponent = <ServerIcon />;
            break;
        case IconType.SHARE:
            iconComponent = <ShareIcon />;
            break;
        case IconType.SHIELD_CHECK:
            iconComponent = <ShieldCheckIcon />;
            break;
        case IconType.SHIELD_X:
            iconComponent = <ShieldXIcon />;
            break;
        case IconType.SQUARE:
            iconComponent = <SquareIcon />;
            break;
        case IconType.SQUARE_CHECK:
            iconComponent = <SquareCheckIcon />;
            break;
        case IconType.SQUARE_SLASH:
            iconComponent = <SquareSlashIcon />;
            break;
        case IconType.TAG:
            iconComponent = <TagIcon />;
            break;
        case IconType.TAGS:
            iconComponent = <TagsIcon />;
            break;
        case IconType.TASKS:
            iconComponent = <KanbanIcon />;
            break;
        case IconType.TERMINAL:
            iconComponent = <TerminalIcon />;
            break;
        default:
            fail('Icon type not recognized');
    }
    return <div className="icon">{iconComponent}</div>;
}
