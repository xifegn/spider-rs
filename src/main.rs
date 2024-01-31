use reqwest;
use tokio;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::get("http://localhost:8000/test").await?;

    if response.status().is_success() {
        let text = response.text().await?;
        println!("text: {}", text);
    } else {
        println!("Error: {}", response.status());
    }

    Ok(())
}