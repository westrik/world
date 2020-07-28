import { h } from 'preact';
import { randomIdentifier } from '~utils/identifier';

export enum FileType {
    EPUB = 'application/epub+zip',
    GIF = 'image/gif',
    JPEG = 'image/jpeg',
    PDF = 'application/pdf',
    PNG = 'image/png',
    MP3 = 'audio/mpeg',
    MPEG = 'video/mpeg',
    SVG = 'image/svg+xml',
    WAV = 'audio/wav',
    WEBM = 'image/webm',
    WEBP = 'image/webp',
    TIFF = 'image/tiff',
    TXT = 'text/plain',
}

interface FileUploadFieldProps {
    labelText: string;
    allowedFileTypes: Array<FileType>;
    disabled?: boolean;
    onChange: (event: Event) => void;
    allowMultiple?: boolean;
}

export default function FileUploadField(props: FileUploadFieldProps): h.JSX.Element {
    const fieldId = randomIdentifier();
    return (
        <fieldset className="upload-field">
            <label htmlFor={fieldId}>{props.labelText}</label>
            <input
                onChange={props.onChange}
                multiple={props.allowMultiple}
                type="file"
                id={fieldId}
                accept={props.allowedFileTypes.join(',')}
            />
        </fieldset>
    );
}
