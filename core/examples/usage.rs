use anyhow::Result;
use devx_core::preferences::Preferences;
use dirs::data_dir;

fn main() -> Result<()> {
    // Create a preferences file.
    let dir = data_dir().unwrap().join("devx");
    let preferences = Preferences::new()
        .author("Eduardo Flores")
        .location(dir.display().to_string());
    preferences.save()?;
    println!("New preferences: {preferences:#?}");

    // Load existing preferences.
    let preferences_load = Preferences::load(&dir.display().to_string())?;
    println!("Loaded current preferences: {preferences_load:#?}");

    // Edit current preferences.
    let edited_preferences = preferences.author("New Author");
    edited_preferences.save()?;
    println!("Edited preferences: {edited_preferences:#?}");

    // Load edited preferences.
    let preferences_load = Preferences::load(&dir.display().to_string())?;
    println!("Loaded edited preferences: {preferences_load:#?}");

    Ok(())
}
