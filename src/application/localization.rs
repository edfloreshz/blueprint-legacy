use i18n_embed::DesktopLanguageRequester;
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DefaultLocalizer, LanguageLoader, Localizer,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader
        .load_fallback_language(&Localizations)
        .expect("Error while loading fallback language");

    loader
});

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        &i18n_embed_fl::fl!($crate::application::localization::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        &i18n_embed_fl::fl!($crate::application::localization::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

// Get the `Localizer` to be used for localizing this library.
pub fn localizer() -> Box<dyn Localizer> {
    Box::new(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

/// To try different localization files replace the function body
/// with this and modify the language code.
///
/// ```
/// use i18n_embed::unic_langid::LanguageIdentifier;
/// let requested_languages = DesktopLanguageRequester::new();
/// let li: LanguageIdentifier = "es-MX".parse().expect("Failed to parse.");
/// if let Err(error) = localizer.select(&[li]) {
///     eprintln!("Error while loading language: {error}");
/// }
/// ```
pub fn init() {
    let localizer = crate::application::localization::localizer();
    let requested_languages = DesktopLanguageRequester::requested_languages();

    if let Err(error) = localizer.select(&requested_languages) {
        eprintln!("Error while loading language: {error}");
    }
}
