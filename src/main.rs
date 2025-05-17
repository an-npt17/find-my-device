use std::error::Error;

mod location;
mod storage;
mod system;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting location tracker...");

    // Get the current location
    let location = location::get_location()?;
    println!("Location: {:?}", location);

    // Store the location with timestamp
    storage::save_location(&location)?;

    // Optional: You might want to set up the auto-start here if not already done
    // system::ensure_autostart()?;

    println!("Location tracked successfully!");
    Ok(())
}
