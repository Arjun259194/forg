use std::process;
use structopt::StructOpt;

fn main() {
    let args = forg::Args::from_args();
    if let Err(e) = forg::run(args.dir) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
