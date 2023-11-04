pub trait Literal {
    fn literal(self: &Self) -> &str;
}