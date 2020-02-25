import { Tag } from './Tag';
import { Resource } from './Resource';

export interface APITaskResponse {
    error: string | null;
    task: Task | null;
}

export interface APITask {
    apiId: string;
    description: string;
    createdAt: Date;
    updatedAt: Date;
    completedAt: Date | null;
    siblingApiId: string | null;
    parentApiId: string | null;
    isCollapsed: boolean;
}

export interface Task extends APITask {
    childTasks: Array<Task>;
    tags?: Array<Tag>;
    resources?: Array<Resource>;
}
