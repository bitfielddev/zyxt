use smol_str::SmolStr;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Comment {
    pub content: SmolStr,
}
