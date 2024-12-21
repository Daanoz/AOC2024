use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct AocAttributes {
    pub year: u32,
}
