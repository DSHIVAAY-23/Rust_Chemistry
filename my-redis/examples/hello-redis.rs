use mini_redis::client;

#[tokio::main]      // The #[tokio::main] function is a macro. It transforms the async fn main() into a synchronous fn main() that initializes a runtime instance and executes the async main function.
// get trnsformed into this
// fn main() {
//     let mut rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async {
//         println!("hello");
//     })
// }
async fn main() -> mini_redis::Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}