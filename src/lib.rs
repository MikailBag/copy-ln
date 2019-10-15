use anyhow::Context;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn ensure_dir(file: &Path) -> anyhow::Result<()> {
    let dir = file.parent().unwrap();
    match fs::create_dir_all(dir) {
        Ok(_) => Ok(()),
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => Ok(()),
            _ => {
                return Err(err)
                    .with_context(|| format!("failed to create directory {}", dir.display()))
            }
        },
    }
}

fn copy_recurse(src: &Path, dest: &Path, skip_exist: bool) -> anyhow::Result<()> {
    use fs_extra::{copy_items, dir};
    let mut copy_options = dir::CopyOptions::new();
    copy_options.skip_exist = skip_exist;

    let src = vec![src];
    let dest = dest.parent().unwrap();
    copy_items(&src, dest, &copy_options)
        .with_context(|| format!("couldn't copy {:?} to {:?} recursively", &src[0], dest))
        .map(drop)
}

fn process(file: &Path, prefix: &Path, skip_exist: bool) -> anyhow::Result<()> {
    let is_symlink = fs::metadata(file)
        .with_context(|| format!("couldn't get metadata on {:?}", file))?
        .file_type()
        .is_symlink();
    if !is_symlink {
        let mut dest = prefix.to_owned();
        dest.push(file.strip_prefix(PathBuf::from("/")).unwrap());
        ensure_dir(&dest).context("failed to create dir")?;
        copy_recurse(file, &dest, skip_exist).context("failed to copy recursively")?;
        return Ok(());
    }
    let target = file
        .read_link()
        .with_context(|| format!("couldn't read symlink {:?} target", &file))?;
    let mut target_full = prefix.to_path_buf();
    target_full.push(target);
    let symlink_path = prefix.join(file);
    let symlink_target = prefix.join(&target_full);
    match std::os::unix::fs::symlink(&symlink_path, &symlink_target) {
        Ok(_) => {}
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists if skip_exist => {}
            _ => {
                return Err(err).with_context(|| {
                    format!(
                        "couldn't create symlink {:?} to {:?}",
                        &symlink_path, &symlink_target
                    )
                })
            }
        },
    }
    process(&target_full, prefix, skip_exist)
}

pub fn copy(prefix: &Path, file: &Path, skip_existing: bool) {
    process(file, prefix, skip_existing);
}
