use arcstr::ArcStr;
use heck::ToTitleCase;
use once_cell::sync::Lazy;
use regex::Regex;
fn join(words: &[&str]) -> String {
    words.concat().to_uppercase()
}
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::convert::Infallible;
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;

static IS_UPPERCASE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\p{Lu}+$").unwrap());
static WORD_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-zA-Z][a-z]*|[0-9]+)").unwrap());
const SEPARATOR: &str = "\0";

//TODO: Let serialization happen using to_string and from_str
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Name(ArcStr);

impl Name {
    
    /// Returns the raw underlying string representation of the name.
    pub fn as_repr_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn segments(&self) -> Vec<&str> {
        self.to_vec()
    }
    
    /// Returns the number of segments in the name.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::Name;
    /// let name = Name::from_slices(&["foo", "bar", "baz", "123"]);
    /// assert_eq!(name.segment_count(), 4);
    /// ```
    /// ```
    /// # use morphir_codemodel::classic::name::Name;
    /// let name = Name::from_str("valueInUSD");
    /// let segments = name.segments();
    /// assert_eq!(name.segment_count(), 5, "The segments for {:?} are {:?}", name.as_repr_str(), segments);
    /// ```
    pub fn segment_count(&self) -> usize {
        self.to_vec().len()
    }

    /// Converts a slice of string slices into a `Name`.
    pub fn from_slices(raw_components: &[&str]) -> Self {
        let components = raw_components.join(SEPARATOR);
        Name(ArcStr::from(components))
    }

    #[inline]
    fn to_vec(&self) -> Vec<&str> {
        self.0.split(SEPARATOR).collect()
    }

    /// Translate a string into a `Name` by splitting it into words. The algorithm is designed to
    /// work with most well-known naming conventions or mix of them. The general rule is that
    /// consecutive letters and numbers are treated as words, upper-case letters and non-alphanumeric
    /// characters start a new word.
    ///
    /// # Examples
    ///
    /// ```
    /// # use morphir_codemodel::classic::name::Name;
    /// let name = Name::from_str("fooBar_baz 123");
    /// assert_eq!(name, Name::from_slices(&["foo", "bar", "baz", "123"]));
    /// ```
    ///
    /// ```
    /// # use morphir_codemodel::classic::name::Name;
    /// let name = Name::from_str("valueInUSD");
    /// assert_eq!(name, Name::from_slices(&["value", "in", "u","s","d"]));
    ///```
    ///
    /// ```
    /// # use morphir_codemodel::classic::name::Name;
    /// let name = Name::from_str("ValueInUSD");
    /// assert_eq!(name, Name::from_slices(&["value", "in", "u","s","d"]));
    /// ```
    ///
    /// ```
    /// # use morphir_codemodel::classic::name::Name;
    /// let name = Name::from_str("ValueInUSD");
    /// assert_eq!(name, Name::from_slices(&["value", "in", "u","s","d"]));
    /// ```
    ///
    /// ```
    /// # use morphir_codemodel::classic::name::Name;
    /// let name = Name::from_str("value_in_USD");
    /// assert_eq!(name, Name::from_slices(&["value", "in", "u","s","d"]));
    /// ```
    pub fn from_str(input: &str) -> Self {
        let mut builder = String::new();
        for m in  WORD_PATTERN.find_iter(input) {
            if builder.len() > 0 {
                builder.push_str(SEPARATOR);
            }
            builder.push_str(m.as_str().to_lowercase().as_str());
        }
        Name(ArcStr::from(builder))
    }

    /// Convert the `Name` into a camel case string.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::Name;
    /// let name = Name::from_slices(&["foo", "bar", "baz", "123"]);
    /// assert_eq!(name.to_camel_case(), "fooBarBaz123");
    /// ```
    /// ```
    /// # use morphir_codemodel::classic::name::Name;
    /// let name = Name::from_slices(&["value", "in", "u","s","d"]);
    /// assert_eq!(name.to_camel_case(), "valueInUSD");
    /// ```
    pub fn to_camel_case(&self) -> String {
        self.to_vec()
            .iter()
            .enumerate()
            .map(|(i, s)| {
                if i == 0 {
                    s.to_string()
                } else {
                    s.to_title_case()
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }

    /// Turns a name into a "list" of human-readable strings. The only difference compared
    /// to `components` is how it handles abbreviations. They will be turned into a single
    /// uppercase word instead of separate single-character words.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::Name;
    /// let name = Name::from(vec!["value", "in", "u","s","d"]);
    /// assert_eq!(name.to_human_words(), vec!["value", "in", "USD"]);
    /// ```
    pub fn to_human_words(&self) -> Vec<String> {
        let words_vec = self.to_vec();
        let words: &[String] = &words_vec.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        fn process(prefix: &[String], abbrev: &[String], suffix: &[String]) -> Vec<String> {
            todo!()
        }

        match &words[..] {
            [word] => {
                if word.len() == 1 {
                    words.to_vec()
                } else {
                    //process(&[], &[], &words)
                    todo!()
                }
            }
            _ => process(&[], &[], &words),
        }
    }

    /// Convert the `Name` into a title case string.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::Name;
    /// let name = Name::from_slices(&["foo", "bar", "baz", "123"]);
    /// assert_eq!(name.to_title_case(), "FooBarBaz123");
    /// ```
    ///```
    /// # use morphir_codemodel::classic::name::Name;
    /// let name = Name::from_slices(&["value", "in", "u","s","d"]);
    /// assert_eq!(name.to_title_case(), "ValueInUSD");
    /// ```
    pub fn to_title_case(&self) -> String {
        self.to_vec() 
            .iter()
            .map(|s| s.to_title_case())
            .collect::<Vec<String>>()
            .join("")
    }
}

// impl From<Name> for Vec<&str> {
//     fn from(name: Name) -> Self {
//         name.to_vec()
//     }
// }

impl From<Name> for Vec<String> {
    fn from(name: Name) -> Self {
        name.to_vec().iter().map(|s| s.to_string()).collect()
    }
}

impl From<&str> for Name {
    fn from(input: &str) -> Self {
        Name::from_str(input)
    }
}

impl From<String> for Name {
    fn from(input: String) -> Self {
        Name::from_str(&input)
    }
}

impl FromIterator<String> for Name {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let mut accum = String::new();
        for word in iter.into_iter().map(|s| s.to_lowercase()) {
            accum.push_str(&word);
            accum.push_str(SEPARATOR);
        }
        Name(ArcStr::from(accum))
    }
}

impl IntoIterator for Name {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .into_iter()
    }
}

// impl <T:Into<NameComponent>> From<Vec<T>> for Name {
//     fn from(input: Vec<T>) -> Self {
//         let components = input.into_iter().map(|c| c.into()).collect();
//         Name(components)
//     }
// }

// impl <T:Into<NameComponent>> From<&[T]> for Name {
//     fn from(input: &[T]) -> Self {
//         let components = input.iter().map(|c| Into::into(c)).collect();
//         Name(components)
//     }
// }
//
// impl <T:Into<NameComponent> + Sized, const N:usize>  From<[T;N]> for Name {
//     fn from(input: [T;N]) -> Self {
//         let components = input.iter().map(|c| c.into()).collect();
//         Name(components)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn join(words: &[&str]) -> String {
        words.concat().to_uppercase()
    }
    #[test]
    fn it_should_be_possible_to_join_abbrevs_into_a_single_string() {
        let words = ["u", "s", "d"];
        let actual = join(&words);
        assert_eq!(actual, "USD")
    }

    #[test]
    fn test_from_string() {
        let name = Name::from("HelloWorld");
        assert_eq!(name.segments(), ["hello", "world"]);
    }

    #[test]
    fn test_from_string_with_snake_cased_string() {
        let name = Name::from("hello_world");
        assert_eq!(name.segments(), ["hello", "world"]);
    }

    #[test]
    fn test_from_slices() {
        let name = Name::from_slices(&["Hello", "World"]);
        assert_eq!(name.segments(), ["Hello", "World"]);
    }

    #[test]
    fn should_support_conversion_from_a_string_slice() {
        let name = Name::from("well-known");
        assert_eq!(name, Name::from_slices(&["well", "known"]))
    }

    #[test]
    fn should_support_conversion_from_a_string() {
        let actual = Name::from("PascalCase".to_string());
        let expected = Name::from_slices(&["pascal", "case"]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn should_support_conversion_to_title_case() {
        let name = Name::from("well-known");
        assert_eq!(name.to_title_case(), "WellKnown")
    }

    #[test]
    fn when_serialized_to_json_should_be_a_json_array() {
        let name = Name::from("Alexander Hamilton");
        let actual = json!(name);
        assert_eq!(actual, json!(["alexander", "hamilton"]));
    }
}
