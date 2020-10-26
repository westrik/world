import { ApiResponse, request, RequestMethod } from '~utils/network';
import { AuthContext } from '~auth/AuthContext';
import { ApiMediaItem, MediaItem } from '~models/MediaItem';

export interface GetMediaItemsResponse extends ApiResponse {
    mediaItems: Array<ApiMediaItem>;
}

export default async function listMediaItems(
    authContext: AuthContext,
    onReceiveResponse: (mediaItems: Array<MediaItem>) => void,
): Promise<void> {
    // TODO: check + save to localStorage
    const response = await request<null, GetMediaItemsResponse>(RequestMethod.GET, '/media-item', authContext);
    // TODO: improve error-handling
    if (response) {
        onReceiveResponse(response.mediaItems.map((item): MediaItem => item));
    }
}
