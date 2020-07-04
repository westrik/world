use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fmt;

#[derive(Debug)]
pub enum ResourceType {
    User,
    Job,
    Note,
    NoteVersion,
    Block,
    Task,
    Tag,
    Link,
    // Diagram,
    // Image,
    // Video,
}

impl fmt::Display for ResourceType {
    // TODO: write macro to add Debug and this impl
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = format!("{:?}", self);
        write!(f, "{}", type_str.to_ascii_lowercase())
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
        assert!(Regex::new(r"^noteversion_[A-Za-z0-9]{8}$")
            .unwrap()
            .is_match(&note_version_id));

        let block_id = generate_resource_identifier(Block);
        assert!(Regex::new(r"^block_[A-Za-z0-9]{8}$")
            .unwrap()
            .is_match(&block_id));

        let tag_id = generate_resource_identifier(Tag);
        assert!(Regex::new(r"^tag_[A-Za-z0-9]{8}$")
            .unwrap()
            .is_match(&tag_id));

        let link_id = generate_resource_identifier(Link);
        assert!(Regex::new(r"^link_[A-Za-z0-9]{8}$")
            .unwrap()
            .is_match(&link_id));

        let job_id = generate_resource_identifier(Job);
        assert!(Regex::new(r"^job_[A-Za-z0-9]{8}$")
            .unwrap()
            .is_match(&job_id));
    }
}
