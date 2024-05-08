use std::ops::Deref;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

pub trait NameRepr: Deref<Target = str> + FromStr {
    type Segment: Into<String>;
    fn segments(&self) -> &[Self::Segment];
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Name<Repr>(Repr) where Repr:NameRepr;

impl <Repr:NameRepr> Name<Repr> {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn repr(&self) -> &Repr {
        &self.0
    }
}
