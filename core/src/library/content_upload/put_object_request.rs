pub fn generate_presigned_upload_url(file_size_bytes: i32) -> String {
    format!("https://upload.example.com/?file_size={}", file_size_bytes)
}
