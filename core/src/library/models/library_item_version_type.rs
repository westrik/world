use std::fmt;

use diesel::sql_types::Text;
use diesel::Expression;

#[derive(Debug, Serialize, Deserialize)]
pub enum LibraryItemVersionType {
    Original,
    PreviewImage,
}

impl fmt::Display for LibraryItemVersionType {
    // TODO: write macro to add Debug and this impl? or improve in some other way
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = format!("{:?}", self);
        write!(f, "{}", type_str.to_ascii_lowercase())
    }
}

impl Expression for LibraryItemVersionType {
    type SqlType = Text;
}