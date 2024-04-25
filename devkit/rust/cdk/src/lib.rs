pub(crate) mod internals;

pub use internals::naming::CanonicalNameStr;
pub use internals::naming::Name;
pub use internals::naming::NamingContext;
pub use internals::naming::Path;

pub mod project {
    pub use super::internals::project::DependencyInfo;
}
