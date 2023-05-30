#[macro_use]
extern crate derive_new;

#[rustfmt::skip]
mod config;
mod app;
mod application;
mod components;
mod factories;
mod modals;
mod models;
mod setup;

use anyhow::Result;
use relm4::RelmApp;

use app::App;
use setup::setup;

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

fn main() -> Result<()> {
    let app = setup()?;

    let app = RelmApp::from_app(app);

    app.run_async::<App>(());

    Ok(())
}
