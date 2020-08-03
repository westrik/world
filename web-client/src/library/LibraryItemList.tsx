import { h } from 'preact';
import { useContext } from 'preact/hooks';

import Auth from '~auth/AuthContext';
import AppContainer from '~components/AppContainer';
import FileUploadField, { FileType } from '~components/FileUploadField';
import bulkCreateLibraryItems from '~library/bulkCreateLibraryItems';

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
                    const files = Array.from((ev.target as HTMLInputElement).files ?? []);
                    bulkCreateLibraryItems(authContext, files);
                }}
            />
        </AppContainer>
    );
}

export default LibraryItemList;
