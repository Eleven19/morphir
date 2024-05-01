use heck::ToTitleCase;
use regex::Regex;
use serde::{Deserialize, Serialize};

//TODO: Let serialization happen using to_string and from_str
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Name(Vec<String>);

impl Name {
    pub fn segments(&self) -> Vec<String> {
        self.0.clone()
    }

    /// Converts a slice of string slices into a `Name`.
    pub fn from_slices(parts: &[&str]) -> Self {
        Name(parts.iter().map(|s| s.to_string()).collect())
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
        let word_pattern: regex::Regex = Regex::new(r"([a-zA-Z][a-z]*|[0-9]+)").unwrap();
        let result = word_pattern
            .find_iter(input)
            .map(|m| m.as_str().to_lowercase())
            .collect();
        Name(result)
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
        self.0.iter().map(|s| s.to_title_case()).collect::<Vec<String>>().join("")
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

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[test]
    fn test_from_string() {
        let name = Name::from("HelloWorld");
        assert_eq!(name.segments(), vec!["hello".to_string(), "world".to_string()]);
    }

    #[test]
    fn test_from_string_with_snake_cased_string() {
        let name = Name::from("hello_world");
        assert_eq!(name.segments(), vec!["hello".to_string(), "world".to_string()]);
    }

    #[test]
    fn test_from_slices() {
        let name = Name::from_slices(&["Hello", "World"]);
        assert_eq!(name.segments(), vec!["Hello".to_string(), "World".to_string()]);
    }

    #[test]
    fn should_support_conversion_from_a_string_slice(){
        let name = Name::from("well-known");
        assert_eq!(name, Name::from_slices(&["well", "known"]))
    }

    #[test]
    fn should_support_conversion_from_a_string(){
        let actual = Name::from("PascalCase".to_string());
        let expected = Name::from_slices(&["pascal", "case"]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn should_support_conversion_to_title_case(){
        let name = Name::from("well-known");
        assert_eq!(name.to_title_case(), "WellKnown")
    }

    #[test]
    fn when_serialized_to_json_should_be_a_json_array(){
        let name = Name::from("Alexander Hamilton");
        let actual = json!(name);
        assert_eq!(actual, json!(["alexander", "hamilton"]));
    }
}
