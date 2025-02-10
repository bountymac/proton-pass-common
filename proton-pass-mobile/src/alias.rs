use proton_pass_common::alias_prefix::AliasPrefixError as CommonAliasPrefixError;

#[derive(Debug, proton_pass_derive::Error, PartialEq, Eq)]
pub enum AliasPrefixError {
    TwoConsecutiveDots,
    InvalidCharacter,
    DotAtTheBeginning,
    DotAtTheEnd,
    PrefixTooLong,
    PrefixEmpty,
}

impl From<CommonAliasPrefixError> for AliasPrefixError {
    fn from(e: CommonAliasPrefixError) -> Self {
        match e {
            CommonAliasPrefixError::TwoConsecutiveDots => Self::TwoConsecutiveDots,
            CommonAliasPrefixError::InvalidCharacter => Self::InvalidCharacter,
            CommonAliasPrefixError::DotAtTheBeginning => Self::DotAtTheBeginning,
            CommonAliasPrefixError::DotAtTheEnd => Self::DotAtTheEnd,
            CommonAliasPrefixError::PrefixTooLong => Self::PrefixTooLong,
            CommonAliasPrefixError::PrefixEmpty => Self::PrefixEmpty,
        }
    }
}

pub struct AliasPrefixValidator;

impl AliasPrefixValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, prefix: String) -> Result<(), AliasPrefixError> {
        Ok(proton_pass_common::alias_prefix::validate_alias_prefix(&prefix)?)
    }
}
