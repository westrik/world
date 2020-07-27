import { h } from 'preact';
import { useContext } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import AppContainer from '~components/AppContainer';
import FileUploadField, { FileType } from '~components/FileUploadField';
import bulkCreateLibraryItems, { FileSizeToLibraryItemsMap } from '~library/bulkCreateLibraryItems';

const FILE_ITEM_TYPES = [FileType.GIF, FileType.JPEG, FileType.PDF, FileType.PNG];

function LibraryItemList(): h.JSX.Element {
    const authContext = useContext(Auth);

    return (
        <AppContainer>
            <FileUploadField
                labelText="Create item"
                allowedFileTypes={FILE_ITEM_TYPES}
                allowMultiple={true}
                onChange={(ev: Event): void => {
                    const fileList: FileList = (ev.target as HTMLInputElement).files!;
                    const files = Array.from(fileList);
                    const fileSizesInBytes = files.map((file) => file.size);
                    // for (let i = 0; i < fileList.length; i++) {
                    //     // TODO: API call with file sizes
                    //     //  - use pre-signed upload URLs in response to upload files
                    //     console.log(`Size: ${fileList[i].size}`);
                    // }
                    bulkCreateLibraryItems(
                        authContext,
                        fileSizesInBytes,
                        (fileSizeToLibraryItems: FileSizeToLibraryItemsMap): void => {
                            console.log('received response for bulkCreateLibraryItems');
                            console.log(fileSizeToLibraryItems);
                        },
                    );
                }}
            />
        </AppContainer>
    );
}

export default LibraryItemList;
