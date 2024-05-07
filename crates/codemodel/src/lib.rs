extern crate core;

pub(crate) mod internals;

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

pub mod workspace {
    pub use super::internals::workspace::*;
}
