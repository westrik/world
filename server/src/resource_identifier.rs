use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fmt;

#[derive(Debug)]
pub enum ResourceType {
    User,
    Task,
    Doc,
    Tag,
    Link,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn generate_resource_identifier(resource_type: ResourceType) -> String {
    let token: String = thread_rng().sample_iter(&Alphanumeric).take(8).collect();
    format!(
        "{}_{}",
        resource_type.to_string().to_ascii_lowercase(),
        token
    )
}

#[cfg(test)]
pub mod test_resource_identifiers {
    use super::*;
    use crate::resource_identifier::ResourceType::*;
    use regex::Regex;

    #[test]
    fn test_id_generation() {
        let user_id = generate_resource_identifier(User);
        assert!(Regex::new(r"user_[A-Za-z0-9]{8}")
            .unwrap()
            .is_match(&user_id));

        let task_id = generate_resource_identifier(Task);
        assert!(Regex::new(r"task_[A-Za-z0-9]{8}")
            .unwrap()
            .is_match(&task_id));

        let doc_id = generate_resource_identifier(Doc);
        assert!(Regex::new(r"doc_[A-Za-z0-9]{8}").unwrap().is_match(&doc_id));

        let tag_id = generate_resource_identifier(Tag);
        assert!(Regex::new(r"tag_[A-Za-z0-9]{8}").unwrap().is_match(&tag_id));

        let link_id = generate_resource_identifier(Link);
        assert!(Regex::new(r"link_[A-Za-z0-9]{8}")
            .unwrap()
            .is_match(&link_id));
    }
}