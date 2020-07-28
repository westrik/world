import { ApiResponse, request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';
import { ApiNoteSummary, Note } from '~models/Note';

export interface GetNotesResponse extends ApiResponse {
    notes: Array<ApiNoteSummary>;
}

export default async function listLibraryItems(
    authContext: AuthContext,
    onReceiveResponse: (notes: Array<Note>) => void,
): Promise<void> {
    // TODO: check + save to localStorage
    const response = await request<null, GetNotesResponse>(RequestMethod.GET, '/note', authContext);
    // TODO: improve error-handling
    if (response) {
        onReceiveResponse(response.notes.map((apiNote): Note => apiNote));
    }
}
