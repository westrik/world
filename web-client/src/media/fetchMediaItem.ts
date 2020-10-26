import { ApiResponse, request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';
import { ApiMediaItem } from '~models/MediaItem';

export interface GetMediaItemResponse extends ApiResponse {
    mediaItem: ApiMediaItem;
}

export default async function fetchMediaItem(
    authContext: AuthContext,
    mediaItemId: string,
    onReceiveResponse: (mediaItem: ApiMediaItem) => void,
): Promise<void> {
    // TODO: check + save to localStorage
    const response = await request<null, GetMediaItemResponse>(
        RequestMethod.GET,
        `/media-item/${mediaItemId}`,
        authContext,
    );
    // TODO: improve error-handling
    if (response) {
        onReceiveResponse(response.mediaItem);
    }
}
