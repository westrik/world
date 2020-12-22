import { ApiResponse, request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';
import { ApiNoteSummary, Note } from '~models/Note';

export interface GetNotesResponse extends ApiResponse {
    notes: Array<ApiNoteSummary>;
}

export default async function listNotes({
    authContext,
    handleReceiveResponse,
}: {
    authContext: AuthContext;
    handleReceiveResponse: (notes: Array<Note>) => void;
}): Promise<void> {
    const response = await request<null, GetNotesResponse>(RequestMethod.GET, '/note', authContext);
    handleReceiveResponse(response.notes.map((apiNote): Note => apiNote));
}
