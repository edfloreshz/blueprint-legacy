use anyhow::Result;
use devx_core::preferences::{
    code_editor::CodeEditor, language::Language, library::Library, shell::Shell, Preferences,
};
use dirs::data_dir;

fn main() -> Result<()> {
    // Create a preferences file.
    let dir = data_dir().unwrap().join("devx");
    let mut preferences = Preferences::new()
        .set_author("Eduardo Flores")
        .set_location(dir.display().to_string())
        .set_shells(vec![Shell::default().set_name("fish").clone()])
        .set_languages(vec![Language::default().set_name("Rust").clone()])
        .set_libraries(Library::from_vec(&["gtk4"]))
        .set_code_editors(vec![CodeEditor::default().set_name("hx").clone()])
        .clone();
    preferences.save()?;
    println!("New preferences: {preferences:#?}");

    // Load existing preferences.
    let preferences_load = Preferences::load(&dir.display().to_string())?;
    println!("Loaded current preferences: {preferences_load:#?}");

    // Edit current preferences.
    let edited_preferences = preferences.set_author("New Author");
    edited_preferences.save()?;
    println!("Edited preferences: {edited_preferences:#?}");

    // Load edited preferences.
    let preferences_load = Preferences::load(&dir.display().to_string())?;
    println!("Loaded edited preferences: {preferences_load:#?}");

    Ok(())
}
