use std::path::PathBuf;

use structopt::StructOpt;
#[derive(StructOpt)]
struct Options {
    #[structopt(long = "prefix", short = "p")]
    prefix: PathBuf,
    #[structopt(long = "file", short = "f")]
    file: Vec<PathBuf>,
    #[structopt(long = "skip-exist", short = "s")]
    skip_existing: bool,
}

fn main() {
    let opt: Options = Options::from_args();
    if opt.file.is_empty() {
        eprintln!("warning: no files to copy specified")
    }
    for f in &opt.file {
        if let Err(e) = copy_ln::copy(f, &opt.prefix, opt.skip_existing) {
            eprintln!("{:?}", e);
        }
    }
}
