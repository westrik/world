import { configure } from '@storybook/preact';
import * as SignInForm from '~auth/SignInForm.stories';
import * as Button from '~components/stories/Button.stories';
import * as CodeEditor from '~components/stories/CodeEditor.stories';
import * as Container from '~components/stories/Container.stories';
import * as Note from '~notes/Note.stories';

configure(() => [SignInForm, Container, Button, CodeEditor, Note], module);
