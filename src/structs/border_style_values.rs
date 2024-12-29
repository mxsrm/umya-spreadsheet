use std::{
    borrow::Cow,
    str::FromStr,
};

use super::EnumTrait;
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum BorderStyleValues {
    DashDot,
    DashDotDot,
    Dashed,
    Dotted,
    Double,
    Hair,
    Medium,
    MediumDashDot,
    MediumDashDotDot,
    MediumDashed,
    None,
    SlantDashDot,
    Thick,
    Thin,
}
impl Default for BorderStyleValues {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for BorderStyleValues {
    #[inline]
    fn get_value_string(&self) -> Cow<str> {
        match &self {
            Self::DashDot => Cow::Borrowed("dashDot"),
            Self::DashDotDot => Cow::Borrowed("dashDotDot"),
            Self::Dashed => Cow::Borrowed("dashed"),
            Self::Dotted => Cow::Borrowed("dotted"),
            Self::Double => Cow::Borrowed("double"),
            Self::Hair => Cow::Borrowed("hair"),
            Self::Medium => Cow::Borrowed("medium"),
            Self::MediumDashDot => Cow::Borrowed("mediumDashDot"),
            Self::MediumDashDotDot => Cow::Borrowed("mediumDashDotDot"),
            Self::MediumDashed => Cow::Borrowed("mediumDashed"),
            Self::None => Cow::Borrowed("none"),
            Self::SlantDashDot => Cow::Borrowed("slantDashDot"),
            Self::Thick => Cow::Borrowed("thick"),
            Self::Thin => Cow::Borrowed("thin"),
        }
    }
}
impl FromStr for BorderStyleValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "dashDot" => Ok(Self::DashDot),
            "dashDotDot" => Ok(Self::DashDotDot),
            "dashed" => Ok(Self::Dashed),
            "dotted" => Ok(Self::Dotted),
            "double" => Ok(Self::Double),
            "hair" => Ok(Self::Hair),
            "medium" => Ok(Self::Medium),
            "mediumDashDot" => Ok(Self::MediumDashDot),
            "mediumDashDotDot" => Ok(Self::MediumDashDotDot),
            "mediumDashed" => Ok(Self::MediumDashed),
            "none" => Ok(Self::None),
            "slantDashDot" => Ok(Self::SlantDashDot),
            "thick" => Ok(Self::Thick),
            "thin" => Ok(Self::Thin),
            _ => Err(()),
        }
    }
}
