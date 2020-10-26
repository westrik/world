import { Tag } from './Tag';
import { Resource } from './Resource';
import { ApiResponse } from '~utils/network';

export interface ApiMediaItemResponse extends ApiResponse {
    note: MediaItem | null;
}

export interface ApiMediaItemSummary {
    id: string;
    createdAt: Date;
    updatedAt: Date;
    name: string;
}

export interface ApiMediaItem extends ApiMediaItemSummary {
    versionId: string;
    preSignedUploadUrl: string;
    uploadedFileSizeBytes: number;
}

export interface MediaItem extends ApiMediaItem {
    assetUrl?: string;
    tags?: Array<Tag>;
    resources?: Array<Resource>;
}

export interface ApiMediaItemVersion {
    id: string;
    createdAt: Date;
    versionType: string;
    assetUrl: string;
    assetFileSizeBytes: number;
}
