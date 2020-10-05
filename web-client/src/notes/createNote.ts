import { ApiResponse, request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';
import { ApiNote } from '~models/Note';

interface CreateNoteRequest {
    name: string;
}

export interface CreateNoteResponse extends ApiResponse {
    note: ApiNote;
}

export default async function createNote(authContext: AuthContext, name: string): Promise<ApiNote | null> {
    // TODO: check + save to localStorage
    const response = await request<CreateNoteRequest, CreateNoteResponse>(RequestMethod.POST, `/note/`, authContext, {
        name,
    });
    // TODO: improve error-handling

    return response?.note ?? null;
}
