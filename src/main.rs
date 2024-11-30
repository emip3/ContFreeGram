mod groqApi;
mod menuInterface;

use menuInterface::runMenu;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    runMenu().await?;
    Ok(())
}


