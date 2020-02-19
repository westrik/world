import { Resource } from './resource';

export interface Tag {
    id: string;
    name: string;
    resource?: string | Resource;
}
