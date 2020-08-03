import { Tag } from './Tag';
import { Resource } from './Resource';
import { ApiResponse } from '~utils/network';

export interface ApiLibraryItemResponse extends ApiResponse {
    note: LibraryItem | null;
}

export interface ApiLibraryItemSummary {
    id: string;
    createdAt: Date;
    updatedAt: Date;
    name: string;
}

export interface ApiLibraryItem extends ApiLibraryItemSummary {
    versionId: string;
    preSignedUploadUrl: string;
    uploadedFileSizeBytes: number;
}

export interface LibraryItem extends ApiLibraryItemSummary {
    // content?: Content;
    tags?: Array<Tag>;
    resources?: Array<Resource>;
}

export interface ApiLibraryItemVersion {
    id: string;
    createdAt: Date;
    versionType: string;
    assetUrl: string;
    assetFileSizeBytes: number;
}
