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
    let extensions: Vec<(&str, &str)> = vec![
        ("png", "Image"),
        ("jpg", "Image"),
        ("jpeg", "Image"),
        ("gif", "Image"),
        ("bmp", "Image"),
        ("svg", "Image"),
        ("txt", "Document"),
        ("pdf", "Document"),
        ("doc", "Document"),
        ("docx", "Document"),
        ("xls", "Document"),
        ("xlsx", "Document"),
        ("ppt", "Document"),
        ("pptx", "Document"),
        ("odt", "Document"),
        ("mp3", "Audio"),
        ("wav", "Audio"),
        ("flac", "Audio"),
        ("ogg", "Audio"),
        ("m4a", "Audio"),
        ("mp4", "Video"),
        ("avi", "Video"),
        ("mkv", "Video"),
        ("mov", "Video"),
        ("wmv", "Video"),
        ("zip", "Archive"),
        ("rar", "Archive"),
        ("tar", "Archive"),
        ("gz", "Archive"),
        ("7z", "Archive"),
        ("c", "Code"),
        ("cpp", "Code"),
        ("h", "Code"),
        ("hpp", "Code"),
        ("py", "Code"),
        ("java", "Code"),
        ("js", "Code"),
        ("sh", "Script"),
        ("sh", "Script"),
        ("bat", "Script"),
        ("ps1", "Script"),
        ("html", "Web"),
        ("css", "Web"),
        ("php", "Web"),
        ("csv", "Spreadsheet"),
        ("exe", "Executable"),
        ("exe", "Executable"),
        ("out", "Executable"),
        ("app", "Executable"),
        ("deb", "Linux_Binaries"),
        ("rpm", "Linux_Binaries"),
    ];

    extensions
        .iter()
        .map(|(ext, cat)| (ext.to_string(), cat.to_string()))
        .collect()
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
