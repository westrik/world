import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import AppContainer from '~components/AppContainer';
import FileUploadField, { FileType } from '~components/FileUploadField';
import bulkCreateLibraryItems from '~library/bulkCreateLibraryItems';
import { LibraryItem } from '~models/LibraryItem';
import listLibraryItems from '~library/listLibraryItems';
import LoadingSpinner from '~components/LoadingSpinner';

const ALLOWED_FILE_TYPES = [FileType.GIF, FileType.JPEG, FileType.PDF, FileType.PNG];

// TODO: POST /authenticate:cloudfront to get auth cookies
// TODO: query for library items

function LibraryItemList(): h.JSX.Element {
    const [items, setItems] = useState<Array<LibraryItem> | null>(null);
    const authContext = useContext(Auth);

    // TODO: refactor into custom hook
    useEffect(() => {
        if (!items) {
            listLibraryItems(authContext, (libraryItems) => {
                if (libraryItems) {
                    setItems(libraryItems);
                } else {
                    setItems([]);
                }
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
                    bulkCreateLibraryItems(authContext, files, (uploadState) => {
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

export default LibraryItemList;
