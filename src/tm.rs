use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Theme {
    pub name: String,
    pub settings: Vec<Setting>,
    pub uuid: String,
    pub license: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Setting {
    NormalSetting(NormalSetting),
    AnonSetting(AnonSetting),
}

#[derive(Serialize)]
pub struct NormalSetting {
    pub name: String,
    pub scope: String,
    pub settings: HashMap<String, String>,
}

#[derive(Serialize)]
pub struct AnonSetting {
    pub settings: AnonSettingSetting,
}

#[derive(Serialize)]
pub struct AnonSettingSetting {
    pub background: String,
    pub caret: String,
    pub foreground: String,
    pub invisibles: String,
    #[serde(rename = "lineHighlight")]
    pub line_highlight: String,
    pub selection: String,
}

impl Theme {
    pub const fn new(name: String) -> Self {
        Self {
            name,
            settings: vec![],
            uuid: String::new(),
            license: String::new(),
        }
    }

    #[allow(clippy::indexing_slicing)]
    pub fn push_anon(&mut self, colors: &HashMap<String, String>) {
        let anon_settings = AnonSettingSetting {
            background: colors["editor.background"].clone(),
            caret: colors["editorCursor.foreground"].clone(),
            foreground: colors["editor.foreground"].clone(),
            invisibles: colors["editorWhitespace.foreground"].clone(),
            line_highlight: colors["editor.lineHighlightBackground"].clone(),
            selection: colors["editor.selectionBackground"].clone(),
        };

        self.settings.push(Setting::AnonSetting(AnonSetting {
            settings: anon_settings,
        }));
    }
}
