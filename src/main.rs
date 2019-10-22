use std::path::PathBuf;

use structopt::StructOpt;
#[derive(StructOpt)]
struct Options {
    #[structopt(long, short = "p")]
    prefix: PathBuf,
    #[structopt(long, short = "f")]
    file: Vec<PathBuf>,
    #[structopt(long, short = "s")]
    skip_exist: bool,
    /// Treat symlinks as regular files
    #[structopt(long, short = "l")]
    ignore_symlinks: bool,
}

fn main() {
    let opt: Options = Options::from_args();
    if opt.file.is_empty() {
        eprintln!("warning: no files to copy specified")
    }
    for f in &opt.file {
        if let Err(e) = copy_ln::copy(&opt.prefix, f, opt.skip_exist, opt.ignore_symlinks) {
            eprintln!("{:?}", e);
        }
    }
}
