use std::{
    fs,
    io::{self},
    path::Path,
};

pub fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_typ = entry.file_type()?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if entry_typ.is_dir() {
            copy_dir(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, dest_path)?;
        }
    }
    Ok(())
}
