use world_core::jobs::errors::JobError;

pub fn populate_email_template() -> Result<String, JobError> {
    let tpl = include_str!("./templates/login_notification/content.html");

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(tpl)
        .unwrap();

    let mut globals = liquid::object!({
        "site_name": "westrikworld",
        "action_url": "https://westrik.world/sign-in",
        "login_url": "https://westrik.world/sign-in",
        "feedback_url": "https://westrik.world/sign-in",
        "trial_extension_url": "https://westrik.world/sign-in",
        "expiration_date": "TODAY",
        "username": "matt",
        "something": "SOMETHING"
    });

    let output = template.render(&globals).unwrap();
    // assert_eq!(output, "Liquid! 2".to_string());

    Ok(output)
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
