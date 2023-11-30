pub trait LiteralStr {
    fn literal(self: &Self) -> &str;
}

pub trait LiteralString {
    fn literal(self: &Self) -> String;
}
