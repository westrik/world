import { h } from 'preact';
import { expect } from 'chai';
import { mount } from 'enzyme';

// This import needs to be in at least one test file (✨ to make Enzyme + TypeScript play nice)
// eslint-disable-next-line @typescript-eslint/no-unused-vars
import Config from '~config';
import TaskRow from '~tasks/TaskRow';
import { API_TASKS } from '~fixtures/Tasks';

describe('TaskRow', () => {
    it('renders', () => {
        const wrapper = mount(
            <TaskRow
                handleDragEnd={(): void => {
                    console.log('drag end');
                }}
                handleDragOver={(): void => {
                    console.log('drag over');
                }}
                handleDragStart={(): void => {
                    console.log('drag start');
                }}
                childTasks={[]}
                onCreateTask={(task): void => {
                    console.log(`create task: ${task}`);
                }}
                {...API_TASKS[0]}
            />,
        );
        expect(wrapper.text()).to.include('task one');
    });
});
