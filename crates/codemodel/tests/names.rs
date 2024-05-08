use cucumber::{given, then, when, World};

#[derive(Debug, Default, World)]
pub struct NamesWorld {
    scenario: Option<Fixture>,
}

#[derive(Debug)]
pub enum Fixture {
    GivenStringName {
        input: String,
    }
}

#[tokio::main]
async fn main() {
    NamesWorld::run("tests/features/names").await;
}
