use relm4_icons::icon_name;

use crate::fl;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Page {
    Shells,
    Languages,
    Libraries,
    CodeEditors,
    Preferences,
}

impl Page {
    pub fn name(&self) -> String {
        match self {
            Page::Shells => fl!("shells").clone(),
            Page::Languages => fl!("languages").clone(),
            Page::Libraries => fl!("libraries").clone(),
            Page::CodeEditors => fl!("code-editors").clone(),
            Page::Preferences => fl!("preferences").clone(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Page::Shells => fl!("shells-description").clone(),
            Page::Languages => fl!("languages-description").clone(),
            Page::Libraries => fl!("libraries-description").clone(),
            Page::CodeEditors => fl!("code-editors-description").clone(),
            Page::Preferences => fl!("preferences-description").clone(),
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
