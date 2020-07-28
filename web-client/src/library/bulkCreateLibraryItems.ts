import { ApiResponse, request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';
import { ApiLibraryItem } from '~models/LibraryItem';

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
        console.log(response.libraryItems);

        // TODO: upload each file to S3 its pre-signed URL
        files.forEach((file) => {
            console.log('TODO upload this file:');
            console.log(file);
        });
    }
}
