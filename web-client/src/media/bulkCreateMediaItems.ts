import { AuthContext } from '~auth/AuthContext';
import { FileType } from '~components/FileUploadField';
import { ApiMediaItem, ApiMediaItemVersion } from '~models/MediaItem';
import { assertCondition } from '~utils/asserts';
import { ApiResponse, request, RequestMethod } from '~utils/network';

interface BulkCreateMediaItemsRequest {
    fileSpecs: Array<[number, FileType]>;
}

export interface BulkCreateMediaItemsResponse extends ApiResponse {
    mediaItems: Array<ApiMediaItem>;
}

interface CreateMediaItemVersionRequest {
    mediaItemId: string;
}

export interface CreateMediaItemVersionResponse extends ApiResponse {
    mediaItemVersion: ApiMediaItemVersion;
}

async function createMediaItem(authContext: AuthContext, item: ApiMediaItem, file: File): Promise<void> {
    assertCondition(
        file && file.size == item.uploadedFileSizeBytes,
        'Expected size of created media item to match file size',
    );
    // TODO: improve error-handling
    const uploadResponse = await fetch(item.preSignedUploadUrl, {
        body: file,
        method: RequestMethod.PUT,
    });
    if (uploadResponse.ok) {
        const createVersionResponse = await request<CreateMediaItemVersionRequest, CreateMediaItemVersionResponse>(
            RequestMethod.POST,
            '/media-item-version',
            authContext,
            {
                mediaItemId: item.id,
            },
        );
        console.log(createVersionResponse);
    } else {
        // TODO: retry upload
    }
}

enum UploadStatus {
    COMPLETE,
    UPLOADING,
}

interface UploadState {
    status: UploadStatus;
    completedFiles?: number;
    totalFiles?: number;
    completedBytes?: number;
    totalBytes?: number;
}

export default async function bulkCreateMediaItems(
    authContext: AuthContext,
    files: Array<File>,
    onStatusChange: (status: UploadState) => void,
): Promise<void> {
    const response = await request<BulkCreateMediaItemsRequest, BulkCreateMediaItemsResponse>(
        RequestMethod.POST,
        `/media-item:bulk-create`,
        authContext,
        {
            fileSpecs: files.map((file) => [file.size, file.type as FileType]),
        },
    );
    onStatusChange({ status: UploadStatus.COMPLETE });
    // TODO: improve error-handling
    if (response) {
        const sortedMediaItems = response.mediaItems.sort((a, b) => a.uploadedFileSizeBytes - b.uploadedFileSizeBytes);
        const sortedFiles = files.sort((a, b) => a.size - b.size); // TODO: rate-limit & batch item creation // TODO: track progress
        sortedMediaItems.map((item, idx) => createMediaItem(authContext, item, sortedFiles[idx]));
    }
}
