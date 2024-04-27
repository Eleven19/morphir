pub mod name;
pub mod path;

pub mod distribution {
    pub enum Distribution {
        Application(),
        Bundle(),
        Library(),
    }
}
