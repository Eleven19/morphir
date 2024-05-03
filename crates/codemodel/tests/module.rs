
use cucumber::{given, then, when, World};

#[derive(Debug, Default,World)]
pub struct ModuleWorld {
    
}

#[tokio::main]
async fn main() {
    ModuleWorld::run("tests/features/module").await;
}