import { configure } from '@storybook/preact';
import * as SignInForm from '~auth/SignInForm.stories';
import * as AppContainer from '~components/stories/AppContainer.stories';
import * as ErrorScreen from '~components/stories/ErrorScreen.stories';
import * as Button from '~components/inputs/stories/Button.stories';
import * as CodeEditor from '~components/inputs/stories/CodeEditor.stories';
import * as Note from '~notes/stories/Note.stories';

configure(() => [SignInForm, AppContainer, CodeEditor, Note, ErrorScreen, Button], module);
