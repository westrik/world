use world_core::notes::export::{html::Html, Render};
use world_core::notes::models::note::Note;
use world_core::notes::schema::Content;
use world_core::settings::sites::models::site_page::LoadedSitePage;

pub async fn render_site(pages_with_notes: Vec<(LoadedSitePage, Note)>) {
    // TODO: use multiple threads for rendering
    for (_site_page, note) in pages_with_notes {
        let content: Content = serde_json::from_value(note.content.unwrap()).unwrap();
        let _html: Html = content.render();
    }
    // TODO: populate page templates
}
