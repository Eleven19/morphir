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

#[given(expr = "a name of '{word}'")]
fn set_name_input(world: &mut NamesWorld, input: String) {
    world.scenario = Some(Fixture::GivenStringName { input });
}

#[tokio::main]
async fn main() {
    NamesWorld::run("tests/features/names").await;
}
