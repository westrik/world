import { ApiResponse, request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';
import { ApiNote } from '~models/Note';

export interface GetNoteResponse extends ApiResponse {
    note: ApiNote;
}

export default async function fetchLibraryItem(
    authContext: AuthContext,
    noteId: string,
    onReceiveResponse: (note: ApiNote) => void,
): Promise<void> {
    // TODO: check + save to localStorage
    const response = await request<null, GetNoteResponse>(RequestMethod.GET, `/note/${noteId}`, authContext);
    // TODO: improve error-handling
    if (response) {
        onReceiveResponse(response.note);
    }
}
