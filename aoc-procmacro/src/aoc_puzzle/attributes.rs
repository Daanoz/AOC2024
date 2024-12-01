use darling::FromMeta;

#[derive(Debug, FromMeta)]
#[darling(and_then = AocAttributes::autocorrect)]
pub struct AocAttributes {
    pub day: Option<u32>,
    #[darling(default)]
    pub render: bool,
}

impl AocAttributes {
    fn autocorrect(self) -> darling::Result<Self> {
        Ok(Self {
            day: validate_day(self.day)?,
            render: self.render,
        })
    }
}

pub fn validate_day(day: Option<u32>) -> Result<Option<u32>, darling::Error> {
    if day.is_some_and(|d| !(1..=25).contains(&d)) {
        Err(darling::Error::custom("day must be between 1 and 25"))
    } else {
        Ok(day)
    }
}
