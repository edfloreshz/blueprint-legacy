use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::Result;
use derive_setters::Setters;
use getset::{CopyGetters, Getters, MutGetters};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Default,
    Getters,
    Setters,
    MutGetters,
    CopyGetters,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
)]
#[setters(prefix = "set_")]
pub struct Sources {
    #[setters(borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    apt: Apt,
    #[setters(borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    dnf: Dnf,
    #[setters(borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    flatpak: Flatpak,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Source {
    Apt,
    Dnf,
    Flatpak,
}

impl Source {
    pub fn command(&self) -> &str {
        match self {
            Source::Apt => "apt",
            Source::Dnf => "dnf",
            Source::Flatpak => "flatpak",
        }
    }

    pub fn is_installed(&self) -> bool {
        let output = std::process::Command::new("which")
            .arg(self.command())
            .output()
            .expect("Failed to execute command");

        output.status.success()
    }

    pub async fn install_package(
        &self,
        package_name: impl ToString,
        password: &impl ToString,
    ) -> Result<String> {
        let mut child = if *self == Source::Flatpak {
            Command::new(self.command())
                .args(["install", "-y", &package_name.to_string()])
                .stdin(Stdio::piped())
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?
        } else {
            Command::new("sudo")
                .args([
                    "-S",
                    self.command(),
                    "install",
                    "-y",
                    &package_name.to_string(),
                ])
                .stdin(Stdio::piped())
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?
        };

        let password = password.to_string();

        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        std::thread::spawn(move || {
            stdin
                .write_all(password.as_bytes())
                .expect("Failed to write to stdin");
        });

        let output = child.wait_with_output().expect("Failed to read stdout");

        let message = if output.status.success() {
            format!("Package '{}' is installed.", package_name.to_string())
        } else {
            format!("Failed to install package: '{}', try checking your current configuration for errors.", package_name.to_string())
        };
        Ok(message)
    }

    pub async fn uninstall_package(
        &self,
        package_name: impl ToString,
        password: &impl ToString,
    ) -> Result<String> {
        let mut child = if *self == Source::Flatpak {
            Command::new(self.command())
                .args(["uninstall", "-y", &package_name.to_string()])
                .stdin(Stdio::piped())
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?
        } else {
            Command::new("sudo")
                .args([
                    "-S",
                    self.command(),
                    "remove",
                    "-y",
                    &package_name.to_string(),
                ])
                .stdin(Stdio::piped())
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?
        };

        let password = password.to_string();

        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        std::thread::spawn(move || {
            stdin
                .write_all(password.as_bytes())
                .expect("Failed to write to stdin");
        });

        let output = child.wait_with_output().expect("Failed to read stdout");

        let message = if output.status.success() {
            format!("Package '{}' is installed.", package_name.to_string())
        } else {
            format!("Failed to install package: '{}', try checking your current configuration for errors.", package_name.to_string())
        };
        Ok(message)
    }
}

#[derive(
    Debug,
    Default,
    Getters,
    Setters,
    MutGetters,
    CopyGetters,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
)]
#[setters(prefix = "set_")]
pub struct Apt {
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    package_name: String,
    #[setters(into, strip_option, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    ppa_repository: Option<String>,
}

#[derive(
    Debug,
    Default,
    Getters,
    Setters,
    MutGetters,
    CopyGetters,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
)]
#[setters(prefix = "set_")]
pub struct Dnf {
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    package_name: String,
    #[setters(into, strip_option, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    copr_repository: Option<String>,
}

#[derive(
    Debug,
    Default,
    Getters,
    Setters,
    MutGetters,
    CopyGetters,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
)]
#[setters(prefix = "set_")]
pub struct Flatpak {
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    app_id: String,
    #[setters(into, strip_option, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    flatpak_repository: Option<String>,
}
