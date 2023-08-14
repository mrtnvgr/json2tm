use clap::Parser;
use std::{fs::File, path::PathBuf};
use tm::{NormalSetting, Setting};

mod tm;
mod vscode;

#[derive(Parser)]
struct Args {
    pub json: PathBuf,
    pub tm: PathBuf,
}

fn main() {
    let args = Args::parse();

    let json_reader = File::open(args.json).unwrap();
    let json: vscode::Theme = serde_json::from_reader(json_reader).unwrap();

    let mut tm = tm::Theme::new(json.name.clone());
    tm.push_anon(&json.colors);

    for token in json.token_colors {
        let scope = token.scope.to_string();

        let setting = NormalSetting {
            name: token.name.unwrap_or_default(),
            scope,
            settings: token.settings,
        };

        tm.settings.push(Setting::NormalSetting(setting));
    }

    plist::to_file_xml(args.tm, &tm).unwrap();
}
