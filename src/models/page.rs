use relm4_icons::icon_name;

#[derive(Debug)]
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

    pub fn icon<'a>(&self) -> &'a str {
        match self {
            Page::Shells => icon_name::CODE_BLOCK_FILLED,
            Page::Languages => icon_name::CHAT_FILLED,
            Page::Libraries => icon_name::BOOKMARK_MULTIPLE_FILLED,
            Page::CodeEditors => icon_name::SLIDE_TEXT_FILLED,
            Page::Preferences => icon_name::SETTINGS_FILLED,
        }
    }
}
