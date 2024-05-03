pub(crate) mod internals;

pub use internals::naming::CanonicalNameStr;
pub use internals::naming::Name;
pub use internals::naming::NamingContext;
pub use internals::naming::PackageName;
pub use internals::naming::Path;

pub mod classic {
    pub use super::internals::classic::*;
}

pub mod project {
    pub use super::internals::project::*;
}

pub mod distribution {
    pub use super::internals::distribution::*;
}

pub mod module {
    pub use super::internals::module::*;
}

pub mod tools {
    pub use super::internals::tools::*;
}