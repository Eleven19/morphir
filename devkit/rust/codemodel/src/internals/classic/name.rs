use regex::Regex;
use serde::{Deserialize, Serialize};

//TODO: Let serialization happen using to_string and from_str
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name(Vec<String>);

impl Name {
    pub fn parts(&self) -> Vec<String> {
        self.0.clone()
    }

    pub fn from_slices(parts: &[&str]) -> Self {
        Name(parts.iter().map(|s| s.to_string()).collect())
    }

    pub fn from_str(input: &str) -> Self {
        let word_pattern: regex::Regex = Regex::new(r"([a-zA-Z][a-z]*|[0-9]+)").unwrap();
        let result = word_pattern
            .find_iter(input)
            .map(|m| m.as_str().to_lowercase())
            .collect();
        Name(result)
    }

    pub fn from_string(s: String) -> Self {
        Name::from_str(s.as_str())
    }
}

impl From<&str> for Name {
    fn from(input: &str) -> Self {
        Name::from_str(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let name = Name::from("HelloWorld");
        assert_eq!(name.parts(), vec!["hello".to_string(), "world".to_string()]);
    }

    #[test]
    fn test_from_string_with_snake_cased_string() {
        let name = Name::from("hello_world");
        assert_eq!(name.parts(), vec!["hello".to_string(), "world".to_string()]);
    }

    #[test]
    fn test_from_slices() {
        let name = Name::from_slices(&["Hello", "World"]);
        assert_eq!(name.parts(), vec!["Hello".to_string(), "World".to_string()]);
    }
}
