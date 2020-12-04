use liquid::Template;

use world_core::notes::export::Render;
use world_core::notes::models::note::Note;
use world_core::notes::schema::Content;
use world_core::settings::sites::models::site_page::LoadedSitePage;

pub struct RenderedPage {
    pub path: String,
    pub content: String,
}

pub async fn render_site(
    site_name: String,
    pages_with_notes: Vec<(LoadedSitePage, Note)>,
) -> Vec<RenderedPage> {
    let mut rendered_pages: Vec<RenderedPage> = Vec::new();
    let pages_with_notes: Vec<(LoadedSitePage, Note)> = pages_with_notes
        .into_iter()
        .filter(|(site_page, _)| site_page.published)
        .collect();
    let posts_data: Vec<PostData> = pages_with_notes
        .iter()
        .cloned()
        .map(|(page, note)| PostData {
            path: page.path,
            name: note.name,
        })
        .collect();
    let list_page = RenderedPage {
        path: "".to_string(),
        content: populate_list_template(site_name.clone(), posts_data),
    };
    rendered_pages.push(list_page);
    let content_pages: Vec<RenderedPage> = pages_with_notes
        // TODO: parallel iter
        .iter()
        .cloned()
        .map(|(site_page, note)| {
            let content: Content = serde_json::from_value(note.content.unwrap()).unwrap();
            let rendered_content = content.render().output;
            let rendered_page =
                populate_page_template(site_name.clone(), note.name, rendered_content);
            RenderedPage {
                path: site_page.path,
                content: rendered_page,
            }
        })
        .collect();
    rendered_pages.extend(content_pages);
    rendered_pages
}

const LIST_TEMPLATE_HTML: &str = include_str!("./templates/blog/index.html");
const POST_TEMPLATE_HTML: &str = include_str!("./templates/blog/post.html");

lazy_static! {
    static ref LIST_TEMPLATE: Template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(LIST_TEMPLATE_HTML)
        .unwrap();
    static ref POST_TEMPLATE: Template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(POST_TEMPLATE_HTML)
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct PostData {
    path: String,
    name: String,
}

fn populate_list_template(site_name: String, posts_data: Vec<PostData>) -> String {
    let template_data = liquid::object!({
        "site_name": site_name,
        "posts": posts_data,
    });
    LIST_TEMPLATE.render(&template_data).unwrap()
}

fn populate_page_template(site_name: String, title: String, content: String) -> String {
    let template_data = liquid::object!({
        "site_name": site_name,
        "title": title,
        "content": content,
    });
    POST_TEMPLATE.render(&template_data).unwrap()
}

#[cfg(test)]
pub mod page_template_population {
    use super::*;

    #[test]
    fn basic() {
        let expected = r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no" />

        <title>Test Post | Test Site</title>

        <link rel="stylesheet" type="text/css" href="./assets/style.css" />
        <!-- <link rel="icon" href="./assets/favicon.svg"> -->
        <!-- <link rel="apple-touch-icon" href="./assets/icon-180.png"> -->
    </head>
    <body>
        <h1>Hello world</h1>
    </body>
</html>
"#;
        let actual = populate_page_template(
            "Test Site".to_string(),
            "Test Post".to_string(),
            "<h1>Hello world</h1>".to_string(),
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn basic_list() {
        let expected = r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no" />

        <title>Test Site</title>

        <link rel="stylesheet" type="text/css" href="./assets/style.css" />
        <!-- <link rel="icon" href="./assets/favicon.svg"> -->
        <!-- <link rel="apple-touch-icon" href="./assets/icon-180.png"> -->
    </head>
    <body>
        <h2>Posts</h2>
        <ul>
            <li><a href="/test-post">Test Post</a></li>
        </ul>
    </body>
</html>
"#;
        let actual = populate_list_template(
            "Test Site".to_string(),
            vec![PostData {
                path: "test-post".to_string(),
                name: "Test Post".to_string(),
            }],
        );
        assert_eq!(expected, actual);
    }
}
