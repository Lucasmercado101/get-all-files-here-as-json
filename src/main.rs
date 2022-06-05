use chrono::Datelike;
use clap::Parser;
use serde_json::{json, Value};
use std::{env::current_dir, fs, path::Path};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Include files in root folder
    /// (the folder that this is executed in)
    #[clap(short, long)]
    include_root: bool,
}

fn get_path_recursively(paths: fs::ReadDir) -> Value {
    let mut root = json!([]);
    for path in paths {
        let path_metadata = fs::metadata(path.as_ref().unwrap().path());

        if path_metadata.unwrap().is_dir() {
            root.as_array_mut()
                .unwrap()
                .push(json!({ path.as_ref().unwrap().file_name().to_str().unwrap(): [get_path_recursively(fs::read_dir(path.as_ref().unwrap().path()).unwrap())] }));
        } else {
            root.as_array_mut().unwrap().push(json!(path
                .as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()));
        }
    }
    root
}

fn main() {
    let args = Args::parse();
    let paths = fs::read_dir(current_dir().unwrap()).unwrap();
    let mut root = json!([]);

    for path in paths {
        let path_metadata = fs::metadata(path.as_ref().unwrap().path());

        if path_metadata.unwrap().is_dir() {
            root.as_array_mut()
                .unwrap()
                .push(json!({ path.as_ref().unwrap().file_name().to_str().unwrap(): [get_path_recursively(fs::read_dir(path.as_ref().unwrap().path()).unwrap())] }));
        } else {
            if args.include_root {
                root.as_array_mut().unwrap().push(json!(path
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()));
            }
        }
    }

    let json_prettified = serde_json::to_string_pretty(&root).unwrap();

    let current_date = chrono::Utc::now();
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day();

    fs::write(
        Path::new(current_dir().unwrap().to_str().unwrap())
            .join(format!("all in here - {} - {} - {}.json", day, month, year)),
        json_prettified,
    )
    .expect("Unable to write file")
}
