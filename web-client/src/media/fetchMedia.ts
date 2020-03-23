import { ApiResponse, request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';
import { ApiNote } from '~models/Note';

export interface GetNotesResponse extends ApiResponse {
    note: ApiNote;
}

export default async function fetchMedia(
    authContext: AuthContext,
    apiId: string,
    onReceiveResponse: (note: ApiNote) => void,
): Promise<void> {
    // TODO: check + save to localStorage
    const response = await request<null, GetNotesResponse>(RequestMethod.GET, `/note/${apiId}`, authContext);
    // TODO: handle errors
    const note = response.note;
    onReceiveResponse(note);
}
