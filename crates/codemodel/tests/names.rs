use cucumber::{given, then, when, World};

#[derive(Debug, Default, World)]
pub struct NamesWorld {}

#[derive(Debug)]
pub enum ScenarioKind{
    GivenStringName()
}

#[tokio::main]
async fn main() {
    NamesWorld::run("tests/features/names").await;
}
