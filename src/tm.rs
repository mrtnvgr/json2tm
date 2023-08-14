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
    pub background: Option<String>,
    pub caret: Option<String>,
    pub foreground: Option<String>,
    pub invisibles: Option<String>,
    #[serde(rename = "lineHighlight")]
    pub line_highlight: Option<String>,
    pub selection: Option<String>,
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
            background: colors.get("editor.background").cloned(),
            caret: colors.get("editorCursor.foreground").cloned(),
            foreground: colors.get("editor.foreground").cloned(),
            invisibles: colors.get("editorWhitespace.foreground").cloned(),
            line_highlight: colors.get("editor.lineHighlightBackground").cloned(),
            selection: colors.get("editor.selectionBackground").cloned(),
        };

        self.settings.push(Setting::AnonSetting(AnonSetting {
            settings: anon_settings,
        }));
    }
}
