use crate::classic::name::Name;
use lazy_regex::{regex, Lazy};
use regex::Regex;
use serde::{Deserialize, Serialize};
static SEPARATOR_REGEX: &Lazy<Regex, fn() -> Regex> = regex!(r"\.");

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Path(Vec<Name>);
impl Path {
    /// Convert a string slice into a `Path`.
    ///
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::path::Path;
    /// let path = Path::from_str("foo.bar.baz");
    /// assert_eq!(path, Path::from_names(vec!["foo", "bar", "baz"]));
    pub fn from_str(input: &str) -> Self {
        let names = SEPARATOR_REGEX
            .split(input)
            .map(|s| Name::from_str(s))
            .collect();
        Path(names)
    }

    pub fn from_names<T>(names: Vec<T>) -> Self
    where
        T: Into<Name>,
    {
        Path(names.into_iter().map(|n| n.into()).collect())
    }

    pub fn names(&self) -> &Vec<Name> {
        &self.0
    }
}
