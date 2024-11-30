use std::io::{self, Write};
use crate::groqApi::groqRequest;
use std::fs::OpenOptions;

pub async fn runMenu() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("\nMenu:");
        println!("1. Send prompt (only answer)");
        println!("2. Send prompt (save answer)");
        println!("3. Exit program");
        print!("Enter your choice: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                match sendPrompt().await {
                    Ok(response) => println!("AI Response: {}", response),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "2" => {
                match sendPrompt().await {
                    Ok(response) => {
                        println!("AI Response: {}", response);
                        if let Err(e) = saveResponse(&response) {
                            println!("Error saving response: {}", e);
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "3" => {
                println!("Exiting program. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Exiting program for safety.");
                break;
            }
        }
    }
    Ok(())
}

async fn sendPrompt() -> Result<String, Box<dyn std::error::Error>> {
    print!("Enter your prompt: ");
    io::stdout().flush()?;
    let mut prompt = String::new();
    io::stdin().read_line(&mut prompt)?;
    groqRequest(&prompt.trim()).await
}

fn saveResponse(response: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("savedResponses.txt")?;
    writeln!(file, "{}", response)?;
    println!("Response saved to savedResponses.txt");
    Ok(())
}


