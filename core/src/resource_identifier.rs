use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ResourceType {
    Job,
    LibraryItem,
    LibraryItemVersion,
    Link,
    Note,
    NoteVersion,
    Site,
    SitePage,
    Tag,
    Task,
    User,
}

lazy_static! {
    static ref RESOURCE_TYPE_TO_PREFIX: HashMap<ResourceType, &'static str> = {
        let mut m = HashMap::new();
        m.insert(ResourceType::NoteVersion, "nv");
        m.insert(ResourceType::LibraryItem, "li");
        m.insert(ResourceType::LibraryItemVersion, "liv");
        m.insert(ResourceType::SitePage, "stpg");
        m
    };
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = format!("{:?}", self);
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

pub fn split_resource_identifier(ident: &str) -> String {
    ident
        .split('_')
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .to_string()
}

#[cfg(test)]
pub mod resource_identifiers {
    use super::*;
    use crate::resource_identifier::ResourceType::*;
    use regex::Regex;

    #[test]
    fn test_id_generation() {
        for (resource_type, prefix) in [
            (Job, "job"),
            (LibraryItem, "li"),
            (LibraryItemVersion, "liv"),
            (Link, "link"),
            (Note, "note"),
            (NoteVersion, "nv"),
            (Site, "site"),
            (SitePage, "stpg"),
            (Tag, "tag"),
            (Task, "task"),
            (User, "user"),
        ]
        .iter()
        {
            let api_id = generate_resource_identifier(*resource_type);
            let api_id_segments: Vec<&str> = api_id.split('_').collect();

            assert_eq!(prefix, api_id_segments.get(0).unwrap());
            assert!(Regex::new(r"^[A-Za-z0-9]{8}$")
                .unwrap()
                .is_match(api_id_segments.get(1).unwrap()));
        }
    }

    #[test]
    fn test_split_resource_identifier() {
        let random_chunk = split_resource_identifier("test_abcd1234");
        assert_eq!(random_chunk, "abcd1234");
    }
}
