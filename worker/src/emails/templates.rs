use tera::{Context, Tera};
use world_core::jobs::errors::JobError;

// lazy_static! {
//     pub static ref TEMPLATES: Tera = {
//         tera
//     };
// }
//
// // "login_notification"
//

pub fn populate_email_template() -> Result<String, JobError> {
    // TODO: render HTML and text

    let mut tera = match Tera::new("./templateasdioafdsikljadfskljs/**/*") {
        Ok(t) => t,
        Err(e) => panic!("Template parsing error(s): {}", e),
    };
    // if let Err(err) = tera.add_template_files(vec![
    //     ("login_notification.html", Some("login_notification")),
    //     ("login_notification.txt", Some("login_notification_txt")),
    // ]) {
    //     panic!("Failed to load template(s): {} [dir={:#?}]", err, std::env::current_dir().unwrap());
    //
    // }
    let mut context = Context::new();
    context.insert("site_name", "westrikworld");
    let rendered = tera
        .render("login_notification.html", &context)
        .map_err(|e| JobError::InternalError(format!("Failed to render template: {}", e)))?;

    Ok(rendered.to_string())
}

#[cfg(test)]
pub mod email_template_population {
    use super::*;

    #[test]
    fn basic() {
        let populated_template = populate_email_template();
        assert_eq!(populated_template.unwrap(), "TEMPLATE".to_string());
    }
}
