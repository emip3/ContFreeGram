mod groq_api;
use groq_api::groq_request;
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    groq_request().await?;
    Ok(())
}
