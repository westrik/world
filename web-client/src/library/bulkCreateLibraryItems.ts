import { AuthContext } from '~auth/AuthContext';
import { ApiLibraryItem } from '~models/LibraryItem';
import { assertCondition } from '~utils/asserts';
import { ApiResponse, request, RequestMethod } from '~utils/network';

interface BulkCreateLibraryItemsRequest {
    fileSizesInBytes: Array<number>;
}

export interface BulkCreateLibraryItemsResponse extends ApiResponse {
    libraryItems: Array<ApiLibraryItem>;
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

        sortedLibraryItems.map(async (item, idx) => {
            const fileToUpload = sortedFiles[idx];
            assertCondition(
                fileToUpload && fileToUpload.size == item.uploadedFileSizeBytes,
                'Expected size of created library item to match file size',
            );
            const response = await fetch(item.preSignedUploadUrl, {
                body: fileToUpload,
                method: RequestMethod.PUT,
            });
            console.log(response);
            // TODO: send API request to create LibraryItemVersion
        });
    }
}
