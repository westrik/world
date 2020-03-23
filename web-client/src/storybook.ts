import { configure } from '@storybook/preact';
import * as Container from '~components/stories/Container.stories';
import * as Tile from '~components/stories/Tile.stories';

configure(() => [Container, Tile], module);
