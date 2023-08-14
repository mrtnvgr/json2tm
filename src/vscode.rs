use serde::Deserialize;
use std::{collections::HashMap, fmt};

#[derive(Deserialize)]
pub struct Theme {
    pub name: String,
    #[serde(rename = "tokenColors")]
    pub token_colors: Vec<Color>,
    pub colors: HashMap<String, String>,
}

#[derive(Deserialize, Clone)]
pub struct Color {
    pub name: Option<String>,
    pub scope: StringOrStringVec,
    pub settings: HashMap<String, String>,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum StringOrStringVec {
    String(String),
    Vec(Vec<String>),
}

impl fmt::Display for StringOrStringVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Vec(strs) => strs.join(", "),
            Self::String(x) => x.clone(),
        };

        write!(f, "{s}")
    }
}
