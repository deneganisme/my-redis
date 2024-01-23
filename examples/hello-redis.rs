use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    
    let mut client = client::connect("127.0.0.1:6378").await?;

    client.set("hello", "world".into()).await?;

    let key = "hello";

    let result = client.get(key).await?;
    println!("Got value from the server for {key}: {:?}", result.unwrap());
    

    Ok(())
}
