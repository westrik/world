import { Tag } from './Tag';
import { Resource } from './Resource';

export interface APITask {
    id: string;
    siblingId?: string;
    parentId?: string;
    modifiedAt?: Date;
    createdAt?: Date;
    completed: boolean;
    completedAt?: Date | null;
    position?: number; // TODO: remove
    description: string;
    tags?: Array<Tag>;
    resources?: Array<Resource>;
}

export interface Task extends APITask {
    childTasks: Array<Task>;
}
