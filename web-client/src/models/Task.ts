import { Tag } from './Tag';
import { Resource } from './Resource';
import { ApiResponse } from '~utils/network';

export interface ApiTaskResponse extends ApiResponse {
    task: Task | null;
}

export interface ApiTask {
    id: string;
    description: string;
    createdAt: Date;
    updatedAt: Date;
    completedAt: Date | null;
    siblingId: string | null;
    parentId: string | null;
    isCollapsed: boolean;
}

export interface Task extends ApiTask {
    childTasks: Array<Task>;
    tags?: Array<Tag>;
    resources?: Array<Resource>;
}
