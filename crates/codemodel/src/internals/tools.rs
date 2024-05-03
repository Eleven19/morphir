use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ToolName(Arc<str>);
impl ToolName {
    pub fn new(name: &str) -> Self {
        Self(Arc::from(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns `true` if the name of the tool is equal to the provided name.
    ///
    /// # Examples
    /// ```
    /// # use morphir_codemodel::tools::ToolName;
    /// let tool_name = ToolName::new("morphir");
    /// assert!(tool_name.is("morphir"));
    /// assert!(!tool_name.is("morphir2"));
    /// ```
    pub fn is(&self, name: &str) -> bool {
        self.as_str() == name
    }

    pub fn morphir() -> Self {
        Self::new("morphir")
    }
}

impl Default for ToolName {
    /// Returns a new instance of `ToolName` with the default value of "morphir".
    ///
    /// # Examples
    /// ```
    /// # use morphir_codemodel::tools::ToolName;
    /// let tool_name = ToolName::default();
    /// assert_eq!(tool_name.as_str(), "morphir");
    /// ```
    fn default() -> Self {
        Self::new("morphir")
    }
}

impl Display for ToolName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub trait Tool {
    fn name(&self) -> ToolName;
}

impl dyn Tool {
    pub fn name<T>(tool: &T) -> ToolName
    where
        T: Tool,
    {
        tool.name()
    }

    pub fn morphir() -> ToolName {
        ToolName::new("morphir")
    }
}

#[derive(Debug)]
pub struct Morphir;
impl Tool for Morphir {
    fn name(&self) -> ToolName {
        ToolName::new("morphir")
    }
}

impl Display for Morphir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "morphir")
    }
}
