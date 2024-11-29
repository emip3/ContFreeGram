mod groqApi;
use groqApi::groqRequest;
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    groqRequest().await?;
    Ok(())
}
