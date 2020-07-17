import { ApiResponse, request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';
import { ApiNote } from '~models/Note';

interface UpdateNoteRequest {
    contentRaw: string;
}

export interface UpdateNoteResponse extends ApiResponse {
    note: ApiNote | null;
}

export default async function updateNote(
    authContext: AuthContext,
    noteId: string,
    content: string,
    onReceiveResponse: (note: ApiNote) => void,
): Promise<void> {
    // TODO: check + save to localStorage
    const response = await request<UpdateNoteRequest, UpdateNoteResponse>(
        RequestMethod.PATCH,
        `/note/${noteId}`,
        authContext,
        {
            contentRaw: content,
        },
    );
    if (response.note) {
        onReceiveResponse(response.note);
    } else {
        // TODO: improve error-handling
        console.log(`Failed to update note: ${response.error}`);
    }
}
