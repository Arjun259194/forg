use colored::Colorize;
use std::{collections::HashMap, path::PathBuf};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "forg",
    about = "This is a cli tool for orgonizing your files is a directory"
)]
pub struct Args {
    #[structopt(parse(from_os_str))]
    pub dir: PathBuf,
}

fn get_extension_map() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("png"), String::from("Image"));
    map.insert(String::from("jpg"), String::from("Image"));
    map.insert(String::from("jpeg"), String::from("Image"));
    map.insert(String::from("gif"), String::from("Image"));
    map.insert(String::from("bmp"), String::from("Image"));
    map.insert(String::from("svg"), String::from("Image"));
    map.insert(String::from("txt"), String::from("Document"));
    map.insert(String::from("pdf"), String::from("Document"));
    map.insert(String::from("doc"), String::from("Document"));
    map.insert(String::from("docx"), String::from("Document"));
    map.insert(String::from("xls"), String::from("Document"));
    map.insert(String::from("xlsx"), String::from("Document"));
    map.insert(String::from("ppt"), String::from("Document"));
    map.insert(String::from("pptx"), String::from("Document"));
    map.insert(String::from("odt"), String::from("Document"));
    map.insert(String::from("mp3"), String::from("Audio"));
    map.insert(String::from("wav"), String::from("Audio"));
    map.insert(String::from("flac"), String::from("Audio"));
    map.insert(String::from("ogg"), String::from("Audio"));
    map.insert(String::from("m4a"), String::from("Audio"));
    map.insert(String::from("mp4"), String::from("Video"));
    map.insert(String::from("avi"), String::from("Video"));
    map.insert(String::from("mkv"), String::from("Video"));
    map.insert(String::from("mov"), String::from("Video"));
    map.insert(String::from("wmv"), String::from("Video"));
    map.insert(String::from("zip"), String::from("Archive"));
    map.insert(String::from("rar"), String::from("Archive"));
    map.insert(String::from("tar"), String::from("Archive"));
    map.insert(String::from("gz"), String::from("Archive"));
    map.insert(String::from("7z"), String::from("Archive"));
    map.insert(String::from("c"), String::from("Code"));
    map.insert(String::from("cpp"), String::from("Code"));
    map.insert(String::from("h"), String::from("Code"));
    map.insert(String::from("hpp"), String::from("Code"));
    map.insert(String::from("py"), String::from("Code"));
    map.insert(String::from("java"), String::from("Code"));
    map.insert(String::from("js"), String::from("Code"));
    map.insert(String::from("sh"), String::from("Script"));
    map.insert(String::from("sh"), String::from("Script"));
    map.insert(String::from("bat"), String::from("Script"));
    map.insert(String::from("ps1"), String::from("Script"));
    map.insert(String::from("html"), String::from("Web"));
    map.insert(String::from("css"), String::from("Web"));
    map.insert(String::from("php"), String::from("Web"));
    map.insert(String::from("csv"), String::from("Spreadsheet"));
    map.insert(String::from("exe"), String::from("Executable"));
    map.insert(String::from("exe"), String::from("Executable"));
    map.insert(String::from("out"), String::from("Executable"));
    map.insert(String::from("app"), String::from("Executable"));
    map.insert(String::from("deb"), String::from("Linux_Binaries"));
    map.insert(String::from("rpm"), String::from("Linux_Binaries"));
    map
}

pub fn run(dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let map = get_extension_map();
    let entries = std::fs::read_dir(&dir)?;
    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_file() {
            let extension = file_path.extension().unwrap_or_default();
            let file_type = map
                .get(extension.to_string_lossy().to_string().trim())
                .unwrap_or(&"Other".to_string())
                .to_string();
            let target_dir = dir.join(&file_type);
            std::fs::create_dir_all(&target_dir)?;
            let targer_path = target_dir.join(file_path.file_name().unwrap());
            std::fs::rename(&file_path, &targer_path)?;
            moved_message(
                file_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                file_type,
            );
        }
    }
    Ok(())
}

fn moved_message(s1: String, s2: String) {
    println!(
        "{} --> {:.20} --> {}",
        "Moving...".bright_yellow(),
        s1.green(),
        s2.bright_blue()
    );
}
