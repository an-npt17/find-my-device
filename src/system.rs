use std::error::Error;
use std::fs;

#[cfg(target_os = "linux")]
pub fn ensure_autostart() -> Result<(), Box<dyn Error>> {
    let home = std::env::var("HOME")?;
    let autostart_dir = format!("{}/.config/autostart", home);
    fs::create_dir_all(&autostart_dir)?;

    let desktop_file = format!("{}/location_tracker.desktop", autostart_dir);
    let current_exe = std::env::current_exe()?.to_string_lossy().to_string();

    let desktop_content = format!(
        "[Desktop Entry]\n\
        Type=Application\n\
        Name=Location Tracker\n\
        Exec={}\n\
        Hidden=false\n\
        NoDisplay=false\n",
        current_exe
    );

    fs::write(desktop_file, desktop_content)?;

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn ensure_autostart() -> Result<(), Box<dyn Error>> {
    // Create a registry entry for autostart
    let current_exe = std::env::current_exe()?.to_string_lossy().to_string();

    // Use PowerShell to set registry key (requires admin privileges)
    // Alternative: create a shortcut in the Startup folder
    let startup_folder =
        shellexpand::tilde("~/AppData/Roaming/Microsoft/Windows/Start Menu/Programs/Startup")
            .to_string();

    // Create a .bat file in the startup folder
    let bat_path = format!("{}/location_tracker.bat", startup_folder);
    let bat_content = format!("start \"\" \"{}\"", current_exe);

    fs::write(bat_path, bat_content)?;

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn ensure_autostart() -> Result<(), Box<dyn Error>> {
    let home = std::env::var("HOME")?;
    let launch_agents_dir = format!("{}/Library/LaunchAgents", home);
    fs::create_dir_all(&launch_agents_dir)?;

    let plist_path = format!(
        "{}/com.yourusername.location_tracker.plist",
        launch_agents_dir
    );
    let current_exe = std::env::current_exe()?.to_string_lossy().to_string();

    let plist_content = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
        <plist version=\"1.0\">\n\
        <dict>\n\
            <key>Label</key>\n\
            <string>com.yourusername.location_tracker</string>\n\
            <key>ProgramArguments</key>\n\
            <array>\n\
                <string>{}</string>\n\
            </array>\n\
            <key>RunAtLoad</key>\n\
            <true/>\n\
        </dict>\n\
        </plist>",
        current_exe
    );

    fs::write(&plist_path, plist_content)?;

    // Load the agent
    Command::new("launchctl")
        .args(&["load", &plist_path])
        .output()?;

    Ok(())
}
