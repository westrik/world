import { h } from 'preact';

import AppContainer from '~components/AppContainer';
import FileUploadField, { FileType } from '~components/FileUploadField';

const FILE_ITEM_TYPES = [FileType.GIF, FileType.JPEG, FileType.PDF, FileType.PNG];

function LibraryItemList(): h.JSX.Element {
    return (
        <AppContainer>
            <FileUploadField
                labelText="Create item"
                allowedFileTypes={FILE_ITEM_TYPES}
                allowMultiple={true}
                onChange={(ev: Event): void => {
                    const fileList: FileList = (ev.target as HTMLInputElement).files!;
                    for (let i = 0; i < fileList.length; i++) {
                        // TODO: API call with file sizes
                        //  - use pre-signed upload URLs in response to upload files
                        console.log(`Size: ${fileList[i].size}`);
                    }
                }}
            />
        </AppContainer>
    );
}

export default LibraryItemList;
