import { ApiResponse, request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';
import { ApiLibraryItem } from '~models/LibraryItem';

interface BulkCreateLibraryItemsRequest {
    fileSizesInBytes: Array<number>;
}

export type FileSizeToLibraryItemsMap = { [fileSizeBytes: number]: Array<ApiLibraryItem> };
export interface BulkCreateLibraryItemsResponse extends ApiResponse {
    fileSizeToLibraryItems: FileSizeToLibraryItemsMap;
}

export default async function bulkCreateLibraryItems(
    authContext: AuthContext,
    fileSizesInBytes: Array<number>,
    onReceiveResponse: (fileSizeToLibraryItems: FileSizeToLibraryItemsMap) => void,
): Promise<void> {
    const response = await request<BulkCreateLibraryItemsRequest, BulkCreateLibraryItemsResponse>(
        RequestMethod.POST,
        `/library-item:bulk-create`,
        authContext,
        {
            fileSizesInBytes,
        },
    );
    // TODO: improve error-handling
    if (response) {
        onReceiveResponse(response.fileSizeToLibraryItems);
    }
}
