use std::{
    fs,
    path::{Path, PathBuf},
};
use structopt::StructOpt;
#[derive(StructOpt)]
struct Options {
    #[structopt(long = "prefix", short = "p")]
    prefix: PathBuf,
    #[structopt(long = "file", short = "f")]
    file: Vec<PathBuf>,
}

fn ensure_dir(file: &Path) {
    let dir = file.parent().unwrap();
    dbg!(dir);
    match fs::create_dir_all(dir) {
        Ok(_) => {}
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => {}
            _ => Err(err).unwrap(),
        }
    }
}

fn process(file: &Path, prefix: &Path) {
    dbg!(file, prefix);
    let is_symlink = fs::metadata(file).unwrap().file_type().is_symlink();
    dbg!(is_symlink);
    if !is_symlink {
        let mut dest = prefix.to_owned();
        dest.push(file.strip_prefix(PathBuf::from("/")).unwrap());
        dbg!(&file, &dest);
        ensure_dir(&dest);
        fs::copy(file, dest).unwrap();
        return;
    }
    let target = file.read_link().unwrap();
    let mut target_full = prefix.to_path_buf();
    dbg!(&target, &target_full);
    target_full.push(target);
    let symlink_path = prefix.join(file);
    let symlink_target = prefix.join(&target_full);
    std::os::unix::fs::symlink(symlink_path, symlink_target).unwrap();
    process(&target_full, prefix);
}

fn main() {
    let opt: Options = Options::from_args();
    for f in &opt.file {
        process(f, &opt.prefix);
    }
}
