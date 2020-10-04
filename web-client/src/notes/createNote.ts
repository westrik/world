import { ApiResponse, request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';
import { ApiNote } from '~models/Note';

interface CreateNoteRequest {
    name: string;
}

export interface CreateNoteResponse extends ApiResponse {
    note: ApiNote;
}

export default async function createNote(
    authContext: AuthContext,
    name: string,
    onReceiveResponse: (note: ApiNote) => void,
): Promise<void> {
    // TODO: check + save to localStorage
    const response = await request<CreateNoteRequest, CreateNoteResponse>(RequestMethod.POST, `/note/`, authContext, {
        name,
    });
    // TODO: improve error-handling
    if (response) {
        onReceiveResponse(response.note);
    }
}
