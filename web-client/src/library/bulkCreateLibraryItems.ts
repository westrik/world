import { AuthContext } from '~auth/AuthContext';
import { ApiLibraryItem, ApiLibraryItemVersion } from '~models/LibraryItem';
import { assertCondition } from '~utils/asserts';
import { ApiResponse, request, RequestMethod } from '~utils/network';

interface BulkCreateLibraryItemsRequest {
    fileSizesInBytes: Array<number>;
}

export interface BulkCreateLibraryItemsResponse extends ApiResponse {
    libraryItems: Array<ApiLibraryItem>;
}

interface CreateLibraryItemVersionRequest {
    libraryItemId: string;
}

export interface CreateLibraryItemVersionResponse extends ApiResponse {
    libraryItemVersion: ApiLibraryItemVersion;
}

async function createLibraryItem(authContext: AuthContext, item: ApiLibraryItem, file: File): Promise<void> {
    assertCondition(
        file && file.size == item.uploadedFileSizeBytes,
        'Expected size of created library item to match file size',
    );
    // TODO: improve error-handling
    const uploadResponse = await fetch(item.preSignedUploadUrl, {
        body: file,
        method: RequestMethod.PUT,
    });
    if (uploadResponse.ok) {
        const createVersionResponse = await request<CreateLibraryItemVersionRequest, CreateLibraryItemVersionResponse>(
            RequestMethod.POST,
            '/library-item-version',
            authContext,
            {
                libraryItemId: item.id,
            },
        );
        console.log(createVersionResponse);
    } else {
        // TODO: retry upload
    }
}

export default async function bulkCreateLibraryItems(authContext: AuthContext, files: Array<File>): Promise<void> {
    const response = await request<BulkCreateLibraryItemsRequest, BulkCreateLibraryItemsResponse>(
        RequestMethod.POST,
        `/library-item:bulk-create`,
        authContext,
        {
            fileSizesInBytes: files.map((file) => file.size),
        },
    );
    // TODO: improve error-handling
    if (response) {
        const sortedLibraryItems = response.libraryItems.sort(
            (a, b) => a.uploadedFileSizeBytes - b.uploadedFileSizeBytes,
        );
        const sortedFiles = files.sort((a, b) => a.size - b.size);
        // TODO: rate-limit & batch item creation
        // TODO: track progress
        sortedLibraryItems.map((item, idx) => createLibraryItem(authContext, item, sortedFiles[idx]));
    }
}
