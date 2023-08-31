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

    #[structopt(short = "d", long = "depth")]
    pub depth: bool,
}

fn get_extension_map() -> HashMap<String, String> {
    let extensions: Vec<(&str, &str)> = vec![
        ("png", "Image"),
        ("webp", "Image"),
        ("jfif", "Image"),
        ("jpg", "Image"),
        ("jpeg", "Image"),
        ("gif", "Image"),
        ("bmp", "Image"),
        ("svg", "Image"),
        ("icon", "Image"),
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
        ("msi", "Executable"),
        ("deb", "Linux_Binaries"),
        ("rpm", "Linux_Binaries"),
    ];

    extensions
        .iter()
        .map(|(ext, cat)| (ext.to_string(), cat.to_string()))
        .collect()
}

pub fn run(config: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let map = get_extension_map();
    let entries = std::fs::read_dir(&config.dir)?;
    for entry in entries {
        let entry = entry?;

        let file_path = entry.path();

        if !file_path.is_file() {
            continue;
        }

        let extension = file_path.extension().unwrap_or_default();

        let file_type = match &config.depth {
            true => extension.to_string_lossy().to_string(),
            false => match map.get(extension.to_string_lossy().to_string().trim()) {
                Some(val) => val.to_string(),
                None => continue,
            },
        };

        let target_dir = &config.dir.join(&file_type);

        if std::fs::create_dir_all(&target_dir).is_err() {
            let msg = format!(
                "failed to create directory for {}",
                file_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            );
            err_msg(&msg);
            continue;
        }

        let targer_path = target_dir.join(file_path.file_name().unwrap_or_default());

        if std::fs::rename(&file_path, &targer_path).is_err() {
            err_msg("Failed to move the file to categorized directory");
            continue;
        }

        success_msg(
            file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_type,
        );
    }
    Ok(())
}

fn err_msg(message: &str) {
    eprintln!("{}", message.red());
}

fn success_msg(s1: String, s2: String) {
    println!(
        "{} --> {:.20} --> {}",
        "Moving...".bright_yellow(),
        s1.green(),
        s2.bright_blue()
    );
}
