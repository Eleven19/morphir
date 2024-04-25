pub mod name;

pub mod distribution {
    pub enum Distribution {
        Application(),
        Bundle(),
        Library(),
    }
}
