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
    #[structopt(long = "skip-exist", short = "s")]
    skip_existing: bool,
}

fn ensure_dir(file: &Path) {
    let dir = file.parent().unwrap();
    match fs::create_dir_all(dir) {
        Ok(_) => {}
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => {}
            _ => panic!("couldn't create directory {:?}: {:?}", dir, err),
        }
    }
}

fn copy_recurse(src: &Path, dest: &Path, skip_exist: bool) {
    use fs_extra::{dir, copy_items};
    let mut copy_options = dir::CopyOptions::new();
    copy_options.skip_exist = skip_exist;

    let src = vec![src];
    let dest = dest.parent().unwrap();
    copy_items(&src, dest, &copy_options).expect(&format!("couldn't copy {:?} to {:?} recursively", &src[0], dest));
}

fn process(file: &Path, prefix: &Path, skip_exist: bool) {
    let is_symlink = fs::metadata(file).expect(&format!("couldn't get metadata on {:?}", file)).file_type().is_symlink();
    if !is_symlink {
        let mut dest = prefix.to_owned();
        dest.push(file.strip_prefix(PathBuf::from("/")).unwrap());
        ensure_dir(&dest);
        copy_recurse(file, &dest, skip_exist);
        return;
    }
    let target = file.read_link().expect(&format!("couldn't read symlink {:?} target", &file));
    let mut target_full = prefix.to_path_buf();
    target_full.push(target);
    let symlink_path = prefix.join(file);
    let symlink_target = prefix.join(&target_full);
    match std::os::unix::fs::symlink(&symlink_path, &symlink_target) {
        Ok(_) => {}
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::AlreadyExists if skip_exist => {}
                _ => panic!("couldn't create symlink {:?} to {:?}", &symlink_path, &symlink_target)
            }
        }
   
    }
    process(&target_full, prefix, skip_exist);
}

fn main() {
    let opt: Options = Options::from_args();
    if opt.file.is_empty() {
        eprintln!("warning: no files to copy specified")
    }
    for f in &opt.file {
        process(f, &opt.prefix, opt.skip_existing);
    }
}
