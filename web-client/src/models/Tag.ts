import { Resource } from './Resource';

export interface Tag {
    id: string;
    name: string;
    resource?: string | Resource;
}
