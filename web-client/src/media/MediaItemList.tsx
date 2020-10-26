import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import AppContainer from '~components/AppContainer';
import FileUploadField, { FileType } from '~components/FileUploadField';
import bulkCreateMediaItems from '~media/bulkCreateMediaItems';
import { MediaItem } from '~models/MediaItem';
import listMediaItems from '~media/listMediaItems';
import LoadingSpinner from '~components/LoadingSpinner';
import { ApiResponse, request, RequestMethod } from '~utils/network';

const ALLOWED_FILE_TYPES = [FileType.GIF, FileType.JPEG, FileType.PDF, FileType.PNG];

// TODO: POST /authenticate:cloudfront to get auth cookies
// TODO: query for media items

// eslint-disable-next-line @typescript-eslint/no-empty-interface
interface CfAuthRequest {}
interface CfAuthResponse extends ApiResponse {
    expiresAt: string;
}

function MediaItemList(): h.JSX.Element {
    const [items, setItems] = useState<Array<MediaItem> | null>(null);
    const [authed, setAuthed] = useState(false);
    const authContext = useContext(Auth);

    async function cloudfrontAuthenticate(): Promise<void> {
        const response = await request<CfAuthRequest, CfAuthResponse>(
            RequestMethod.POST,
            '/authenticate:cloudfront',
            authContext,
            {},
            'include',
        );
        if (response?.expiresAt) {
            setAuthed(true);
        }
    }

    useEffect(() => {
        if (!authed) {
            cloudfrontAuthenticate();
        }
        // TODO: refactor into custom hook
        if (!items) {
            listMediaItems(authContext, (mediaItems) => {
                setItems(mediaItems ?? []);
            });
        }
    });

    return (
        <AppContainer>
            <FileUploadField
                labelText="Create item"
                allowedFileTypes={ALLOWED_FILE_TYPES}
                allowMultiple={true}
                onChange={(ev: Event): void => {
                    const files = Array.from((ev.target as HTMLInputElement).files ?? []);
                    bulkCreateMediaItems(authContext, files, (uploadState) => {
                        console.log(`upload is ${uploadState.status}`);
                    });
                }}
            />
            {items ? (
                <ul>
                    {items.map((item, key) => (
                        <li key={key}>
                            {item.assetUrl ? (
                                item.assetUrl.endsWith('png') ||
                                item.assetUrl.endsWith('jpg') ||
                                item.assetUrl.endsWith('gif') ? (
                                    <img alt={item.name} src={item.assetUrl} />
                                ) : (
                                    <a href={item.assetUrl}>{item.name}</a>
                                )
                            ) : (
                                <strong>{item.name}</strong>
                            )}
                        </li>
                    ))}
                </ul>
            ) : (
                <LoadingSpinner />
            )}
        </AppContainer>
    );
}

export default MediaItemList;
