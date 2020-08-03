use heck::SnakeCase;

pub trait ToIdentifier {
    fn to_ident(&self) -> String;
}

impl ToIdentifier for String {
    fn to_ident(&self) -> String {
        self.to_snake_case()
    }
}
