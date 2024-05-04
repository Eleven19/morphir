pub mod deprecated;
pub mod fq_name;
pub mod module;
pub mod name;
pub mod package;
pub mod path;

pub mod distribution {
    pub enum Distribution {
        Application(),
        Bundle(),
        Library(),
    }
}
