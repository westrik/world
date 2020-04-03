import { configure } from '@storybook/preact';
import * as SignInForm from '~auth/SignInForm.stories';
import * as Button from '~components/stories/Button.stories';
import * as Container from '~components/stories/Container.stories';
import * as Tile from '~components/stories/Tile.stories';

configure(() => [SignInForm, Container, Button, Tile], module);
