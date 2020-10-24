/* eslint-disable @typescript-eslint/ban-ts-comment */

import { h } from 'preact';

import { fail } from '~utils/asserts';

// @ts-ignore
import BookmarkIcon from '../static/img/icons/bookmark.svg';
// @ts-ignore
import ChevronCompactLeftIcon from '../static/img/icons/chevron-compact-left.svg';
// @ts-ignore
import ChevronCompactRightIcon from '../static/img/icons/chevron-compact-right.svg';
// @ts-ignore
import ChevronDownIcon from '../static/img/icons/chevron-down.svg';
// @ts-ignore
import ChevronUpIcon from '../static/img/icons/chevron-up.svg';
// @ts-ignore
import CollectionIcon from '../static/img/icons/collection.svg';
// @ts-ignore
import DiagramIcon from '../static/img/icons/diagram-3.svg';
// @ts-ignore
import ExclamationIcon from '../static/img/icons/exclamation.svg';
// @ts-ignore
import FilterIcon from '../static/img/icons/filter.svg';
// @ts-ignore
import GearIcon from '../static/img/icons/gear.svg';
// @ts-ignore
import HomeIcon from '../static/img/icons/house.svg';
// @ts-ignore
import MarkdownIcon from '../static/img/icons/markdown.svg';
// @ts-ignore
import QuestionIcon from '../static/img/icons/question.svg';
// @ts-ignore
import RssIcon from '../static/img/icons/rss.svg';
// @ts-ignore
import SearchIcon from '../static/img/icons/search.svg';
// @ts-ignore
import ServerIcon from '../static/img/icons/server.svg';
// @ts-ignore
import ShareIcon from '../static/img/icons/share.svg';
// @ts-ignore
import ShieldCheckIcon from '../static/img/icons/shield-check.svg';
// @ts-ignore
import ShieldXIcon from '../static/img/icons/shield-x.svg';
// @ts-ignore
import SquareIcon from '../static/img/icons/square.svg';
// @ts-ignore
import SquareCheckIcon from '../static/img/icons/square-check.svg';
// @ts-ignore
import SquareSlashIcon from '../static/img/icons/square-slash.svg';
// @ts-ignore
import TagIcon from '../static/img/icons/tag.svg';
// @ts-ignore
import TagsIcon from '../static/img/icons/tags.svg';
// @ts-ignore
import TasksIcon from '../static/img/icons/kanban.svg';
// @ts-ignore
import TerminalIcon from '../static/img/icons/terminal.svg';

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
            return <DiagramIcon />;
        case IconType.EXCLAMATION:
            return <ExclamationIcon />;
        case IconType.FILTER:
            return <FilterIcon />;
        case IconType.GEAR:
            return <GearIcon />;
        case IconType.HOME:
            return <HomeIcon />;
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
            return <TasksIcon />;
        case IconType.TERMINAL:
            return <TerminalIcon />;
        default:
            fail('Icon type not recognized');
    }
}
