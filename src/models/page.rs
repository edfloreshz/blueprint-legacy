use relm4_icons::icon_name;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Page {
    Shells,
    Languages,
    Libraries,
    CodeEditors,
    Preferences,
}

impl Page {
    pub fn name<'a>(&self) -> &'a str {
        match self {
            Page::Shells => "Shells",
            Page::Languages => "Languages",
            Page::Libraries => "Libraries",
            Page::CodeEditors => "Code editors",
            Page::Preferences => "Preferences",
        }
    }

    pub fn description<'a>(&self) -> &'a str {
        match self {
            Page::Shells => "Select your favorite shells",
            Page::Languages => "Select your favorite programming languages",
            Page::Libraries => "Select all the libraries you need",
            Page::CodeEditors => "Select your favorite code editors",
            Page::Preferences => "Tweak the app and set your preferences",
        }
    }

    pub fn icon<'a>(&self) -> &'a str {
        match self {
            Page::Shells => icon_name::CODE_BLOCK_FILLED,
            Page::Languages => icon_name::CHAT_FILLED,
            Page::Libraries => icon_name::BOOKMARK_MULTIPLE_FILLED,
            Page::CodeEditors => icon_name::SLIDE_TEXT_FILLED,
            Page::Preferences => icon_name::TOGGLE_MULTIPLE_FILLED,
        }
    }
}
