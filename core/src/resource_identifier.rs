use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ResourceType {
    User,
    Job,
    Note,
    NoteVersion,
    Task,
    Tag,
    Link,
    LibraryItem,
    LibraryItemVersion,
}

lazy_static! {
    static ref RESOURCE_TYPE_TO_PREFIX: HashMap<ResourceType, &'static str> = {
        let mut m = HashMap::new();
        m.insert(ResourceType::NoteVersion, "nv");
        m.insert(ResourceType::LibraryItem, "li");
        m.insert(ResourceType::LibraryItemVersion, "liv");
        m
    };
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = self.to_string();
        if let Some(prefix) = (*RESOURCE_TYPE_TO_PREFIX).get(self) {
            write!(f, "{}", prefix)
        } else {
            write!(f, "{}", type_str.to_ascii_lowercase())
        }
    }
}

pub fn generate_resource_identifier(resource_type: ResourceType) -> String {
    let token: String = thread_rng().sample_iter(&Alphanumeric).take(8).collect();
    format!("{}_{}", resource_type.to_string(), token)
}

#[cfg(test)]
pub mod resource_identifiers {
    use super::*;
    use crate::resource_identifier::ResourceType::*;
    use regex::Regex;

    #[test]
    fn id_generation() {
        let user_id = generate_resource_identifier(User);
        assert!(Regex::new(r"^user_[A-Za-z0-9]{8}$")
            .unwrap()
            .is_match(&user_id));

        let task_id = generate_resource_identifier(Task);
        assert!(Regex::new(r"^task_[A-Za-z0-9]{8}$")
            .unwrap()
            .is_match(&task_id));

        let note_id = generate_resource_identifier(Note);
        assert!(Regex::new(r"^note_[A-Za-z0-9]{8}$")
            .unwrap()
            .is_match(&note_id));

        let note_version_id = generate_resource_identifier(NoteVersion);
        assert!(Regex::new(r"^nv_[A-Za-z0-9]{8}$")
            .unwrap()
            .is_match(&note_version_id));

        let tag_id = generate_resource_identifier(Tag);
        assert!(Regex::new(r"^tag_[A-Za-z0-9]{8}$")
            .unwrap()
            .is_match(&tag_id));

        let link_id = generate_resource_identifier(Link);
        assert!(Regex::new(r"^link_[A-Za-z0-9]{8}$")
            .unwrap()
            .is_match(&link_id));

        let li_id = generate_resource_identifier(LibraryItem);
        assert!(Regex::new(r"^li_[A-Za-z0-9]{8}$").unwrap().is_match(&li_id));

        let liv_id = generate_resource_identifier(LibraryItemVersion);
        assert!(Regex::new(r"^liv_[A-Za-z0-9]{8}$")
            .unwrap()
            .is_match(&liv_id));

        let job_id = generate_resource_identifier(Job);
        assert!(Regex::new(r"^job_[A-Za-z0-9]{8}$")
            .unwrap()
            .is_match(&job_id));
    }
}
