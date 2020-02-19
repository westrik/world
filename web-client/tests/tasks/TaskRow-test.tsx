import { h } from 'preact';
import { expect } from 'chai';
import { mount } from 'enzyme';

import Config from '~config';
import TaskRow from '~tasks/TaskRow';

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
                id={'task_abc123'}
                completed={false}
                childTasks={[]}
                description={'task one'}
            />,
        );
        expect(wrapper.text()).to.include('task one');
    });
});
