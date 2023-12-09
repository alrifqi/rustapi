mod logger;

#[tokio::main]
async fn main() {
    logger::setup();
    println!("Hello, world!");
}
