use std::error::Error;

mod location;
mod storage;
mod system;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting location tracker...");

    let location = location::get_location()?;
    println!("Location: {:?}", location);

    storage::save_location(&location)?;

    system::ensure_autostart()?;

    println!("Location tracked successfully!");
    Ok(())
}
