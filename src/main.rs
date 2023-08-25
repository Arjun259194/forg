use std::{path::PathBuf, process};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "forg",
    about = "This is a cli tool for orgonizing your files is a directory"
)]
struct Args {
    #[structopt(parse(from_os_str))]
    dir: PathBuf,
}

fn run(dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let entries = std::fs::read_dir(&dir)?;
    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_file() {
            let extension = file_path.extension().unwrap_or_default();
            let target_dir = dir.join(extension);
            std::fs::create_dir_all(&target_dir)?;
            let targer_path = target_dir.join(file_path.file_name().unwrap());
            std::fs::rename(&file_path, &targer_path)?;
            println!(
                "{0} Moved to {1}",
                file_path.display(),
                targer_path.display()
            );
        }
    }
    Ok(())
}

fn main() {
    let args = Args::from_args();
    if let Err(e) = run(args.dir) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
