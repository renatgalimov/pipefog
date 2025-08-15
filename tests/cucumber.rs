use cucumber::World as _;

#[derive(cucumber::World, Debug, Default)]
struct TestWorld;

#[tokio::main]
async fn main() {
    TestWorld::run("features").await;
}
