use std::fmt;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum FileType {
    #[serde(rename = "application/epub+zip")]
    EPUB,
    #[serde(rename = "image/gif")]
    GIF,
    #[serde(rename = "image/jpeg")]
    JPEG,
    #[serde(rename = "audio/mpeg")]
    MP3,
    #[serde(rename = "video/mpeg")]
    MPEG,
    #[serde(rename = "application/pdf")]
    PDF,
    #[serde(rename = "image/png")]
    PNG,
    #[serde(rename = "image/svg+xml")]
    SVG,
    #[serde(rename = "image/tiff")]
    TIFF,
    #[serde(rename = "text/plain")]
    TXT,
    #[serde(rename = "audio/wav")]
    WAV,
    #[serde(rename = "image/webm")]
    WEBM,
    #[serde(rename = "image/webp")]
    WEBP,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = match self {
            FileType::EPUB => "epub",
            FileType::GIF => "gif",
            FileType::JPEG => "jpg",
            FileType::MP3 => "mp3",
            FileType::MPEG => "mp4",
            FileType::PDF => "pdf",
            FileType::PNG => "png",
            FileType::SVG => "svg",
            FileType::TIFF => "tiff",
            FileType::TXT => "txt",
            FileType::WAV => "wav",
            FileType::WEBM => "webm",
            FileType::WEBP => "webp",
        };
        write!(f, "{}", type_str)
    }
}
