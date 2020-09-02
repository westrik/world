import { ApiResponse, request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';
import { ApiLibraryItem, LibraryItem } from '~models/LibraryItem';

export interface GetLibraryItemsResponse extends ApiResponse {
    libraryItems: Array<ApiLibraryItem>;
}

export default async function listLibraryItems(
    authContext: AuthContext,
    onReceiveResponse: (libraryItems: Array<LibraryItem>) => void,
): Promise<void> {
    // TODO: check + save to localStorage
    const response = await request<null, GetLibraryItemsResponse>(RequestMethod.GET, '/library-item', authContext);
    // TODO: improve error-handling
    if (response) {
        onReceiveResponse(response.libraryItems.map((item): LibraryItem => item));
    }
}
