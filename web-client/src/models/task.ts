import { Tag } from './tag';
import { Resource } from './resource';

export interface APITask {
    id: string;
    parentId?: string;
    position: number;
    description: string;
    completed: boolean;
    tags: Array<Tag>;
    resources: Array<Resource>;
}

export interface Task extends APITask {
    children: Array<Task>;
}
