use arcstr::ArcStr;
use heck::ToTitleCase;
use regex::Regex;
use once_cell::sync::Lazy;fn join(words: &[&str]) -> String {
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

/// A component of a name. A name is essentially a sequence of components.
#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NameComponent(ArcStr);

impl NameComponent {

    /// Append a string to the name component.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::NameComponent;
    /// let name = NameComponent::from("Compound");
    /// assert_eq!(name.append("Word"), NameComponent::from("CompoundWord"));
    /// ```
    pub fn append<T:Display>(&self, other: T) -> NameComponent {
        let new_str = format!("{}{}", self.0, other);
        NameComponent(ArcStr::from(new_str.as_str()))
    }

    /// Extract a string slice containing our data.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::NameComponent;
    /// let name = NameComponent::from("Foo");
    /// assert_eq!(name.as_str(), "Foo");
    /// ```
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    /// Concatenate two name components.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::NameComponent;
    /// let name1 = NameComponent::from("Foo");
    /// let name2 = NameComponent::from("Bar");
    /// assert_eq!(name1.concat(&name2), NameComponent::from("FooBar"));
    /// ```
    pub fn concat(&self, other: &NameComponent) -> NameComponent {
        let new_str = format!("{}{}", self.0, other.0);
        NameComponent(ArcStr::from(new_str.as_str()))
    }

    /// Join a slice of name components using a separator.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::NameComponent;
    /// let name1 = NameComponent::from("Foo");
    /// let name2 = NameComponent::from("Bar");
    /// let name3 = NameComponent::from("Baz");
    /// let components = vec![&name1, &name2, &name3];
    /// assert_eq!(NameComponent::join(&components, "_"), NameComponent::from("Foo_Bar_Baz"));
    /// ```
    pub fn join(components: &[&NameComponent], separator:&str) -> NameComponent {
        let new_str = components.iter().map(|c| c.0.as_ref()).collect::<Vec<&str>>().join(separator);
        NameComponent(ArcStr::from(new_str.as_str()))
    }

    /// Returns our length in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Indicates if the name component is empty.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::NameComponent;
    /// assert!(NameComponent::from("").is_empty());
    /// ```
    /// ```
    /// # use morphir_codemodel::classic::name::NameComponent;
    /// assert!(!NameComponent::from("here").is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Indicates if the name component is title cased.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::NameComponent;
    /// let name = NameComponent::from("Foo");
    /// assert!(name.is_title_cased());
    /// ```
    pub fn is_title_cased(&self) -> bool {
        self.0.chars().next().unwrap().is_uppercase()
    }

    /// Indicates if the name component is upper-cased.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::NameComponent;
    /// let name = NameComponent::from("FOO");
    /// assert!(name.is_uppercase());
    /// ```
    /// ```
    /// # use morphir_codemodel::classic::name::NameComponent;
    /// let name = NameComponent::from("Foo");
    /// assert!(!name.is_uppercase());
    /// ```
    pub fn is_uppercase(&self) -> bool {
        IS_UPPERCASE.is_match(self.0.as_ref())
    }

    /// Prepend a string to the name component.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::NameComponent;
    /// let name = NameComponent::from("Word");
    /// assert_eq!(name.prepend("Compound"), NameComponent::from("CompoundWord"));
    /// ```
    pub fn prepend<T:Display>(&self, other: T) -> NameComponent {
        let new_str = format!("{}{}", other, self.0);
        NameComponent(ArcStr::from(new_str.as_str()))
    }

    pub fn to_lowercase(&self) -> NameComponent {
        self.0.to_lowercase().into()
    }

    pub fn to_title_case(&self) -> String {
        self.0.to_title_case()
    }

    pub fn to_uppercase(&self) -> NameComponent {
        self.0.to_uppercase().into()
    }

    /// Convert the name component into a `std::string::String`.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::NameComponent;
    /// let name = NameComponent::from("Peekaboo");
    /// assert_eq!(name.to_string(), "Peekaboo");
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl AsRef<str> for NameComponent {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<[u8]> for NameComponent {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Borrow<str> for NameComponent {
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::fmt::Debug for NameComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(self.0.as_str(), f)
    }
}

impl Display for NameComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl Deref for NameComponent {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for NameComponent {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NameComponent(ArcStr::from(s)))
    }
}

impl From<&NameComponent> for NameComponent {
    fn from(input: &NameComponent) -> Self {
        input.clone()
    }
}

impl From<String> for NameComponent {
    fn from(input: String) -> Self {
        NameComponent(ArcStr::from(input.as_str()))
    }
}

impl From<&String> for NameComponent {
    fn from(input: &String) -> Self {
        NameComponent(ArcStr::from(input.as_str()))
    }
}

impl From<ArcStr> for NameComponent {
    fn from(input: ArcStr) -> Self {
        NameComponent(input)
    }
}

impl From<NameComponent> for ArcStr {
    fn from(input: NameComponent) -> Self {
        input.0
    }
}

impl From<&str> for NameComponent {
    fn from(input: &str) -> Self {
        NameComponent(ArcStr::from(input))
    }
}

impl <'a> PartialEq<&'a str> for NameComponent {
    fn eq(&self, other: &&'a str) -> bool {
        self.0 == *other
    }
}

//TODO: Let serialization happen using to_string and from_str
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Name(Rc<[NameComponent]>);

impl Name {
    pub fn components(&self) -> &[NameComponent] {
        self.0.deref()
    }

    /// Converts a slice of string slices into a `Name`.
    pub fn from_slices(parts: &[&str]) -> Self {
        let components = parts.iter().map(|s| NameComponent::from(*s)).collect();
        Name(components)
    }

    fn to_str_vec(&self) -> Vec<&str> {
        self.0.iter().map(|c| c.as_str()).collect()
    }

    fn to_arcstrs(&self) -> Vec<ArcStr> {
        self.0.iter().map(|c| c.0.clone()).collect()
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
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-zA-Z][a-z]*|[0-9]+)").unwrap());
        let result = RE
            .find_iter(input)
            .map(|m| NameComponent::from(m.as_str()).to_lowercase())
            .collect();
        Name(result)
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
        self.0
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
        let words:&[String] = words_vec.as_ref();
        fn process(prefix: &[String], abbrev:&[String], suffix:&[String]) -> Vec<String> {
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
             _ => process(&[], &[], &words)
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
        self.0
            .iter()
            .map(|s| s.to_title_case())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn into_boxed_slice_of_string(self) -> Box<[String]> {
        self.0
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .into_boxed_slice()
    }

    pub fn to_vec(&self) -> Vec<String> {
        let res = self.0.to_vec();
        res.iter().map(|c| c.to_string()).collect::<Vec<String>>()
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

impl <T:Into<NameComponent>> From<Vec<T>> for Name {
    fn from(input: Vec<T>) -> Self {
        let components = input.into_iter().map(|c| c.into()).collect();
        Name(components)
    }
}

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

pub struct SName(ArcStr);
impl SName {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}
impl From<Vec<&str>> for SName {

    /// Convert a vector of strings into a single string.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::SName;
    /// let name = SName::from(vec!["foo", "bar", "baz"]);
    /// assert_eq!(name.as_str(), "foo\0bar\0baz");
    /// ```
    fn from(input: Vec<&str>) -> Self {
        let name = input.join("\0");
        SName(ArcStr::from(name))
    }
}

impl From<SName> for Vec<String> {
    
    /// Convert a single string into a vector of strings.
    /// # Examples
    /// ```
    /// # use morphir_codemodel::classic::name::SName;
    /// let name = SName::from(vec!["foo", "bar", "baz"]);
    /// assert_eq!(Vec::<String>::from(name), vec!["foo", "bar", "baz"]);
    /// ```
    fn from(input: SName) -> Vec<String> {
        input.as_str().split('\0').map(|s| s.to_string()).collect()
    }
}

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
        assert_eq!(name.components(), ["hello", "world"]);
    }

    #[test]
    fn test_from_string_with_snake_cased_string() {
        let name = Name::from("hello_world");
        assert_eq!(name.components(), ["hello", "world"]);
    }

    #[test]
    fn test_from_slices() {
        let name = Name::from_slices(&["Hello", "World"]);
        assert_eq!(name.components(), ["Hello", "World"]);
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
