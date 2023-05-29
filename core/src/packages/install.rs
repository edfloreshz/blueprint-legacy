use crate::preferences::source::{Source, Sources};
use anyhow::Result;
use getset::Getters;

#[derive(Debug, Default, Getters)]
pub struct Package {
    #[getset(get = "pub", get_mut = "pub")]
    install_summary: Vec<String>,
}

impl Package {
    pub async fn install_from_sources(
        &mut self,
        sources: &Sources,
        password: &impl ToString,
    ) -> Result<()> {
        let message = if Source::Apt.is_installed() && !sources.apt().package_name().is_empty() {
            Source::Apt
                .install_package(sources.apt().package_name(), password)
                .await?
        } else if Source::Dnf.is_installed() && !sources.dnf().package_name().is_empty() {
            Source::Dnf
                .install_package(sources.dnf().package_name(), password)
                .await?
        } else if Source::Flatpak.is_installed() && !sources.flatpak().app_id().is_empty() {
            Source::Flatpak
                .install_package(sources.flatpak().app_id(), password)
                .await?
        } else {
            "None of the package managers are installed or the package was not correctly configured"
                .into()
        };
        self.install_summary.push(message);
        Ok(())
    }
}
