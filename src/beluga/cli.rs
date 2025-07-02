use std::{io, path::Path};

use crate::beluga::utils::copy_dir;

/// We have three commands right here
///
/// init
///
/// watch
///
/// help

pub fn create(site_name: &String) -> io::Result<()> {
    println!("creating new project {site_name}");
    let current_path = std::env::current_dir()?;
    let current_path = current_path.join(site_name);
    copy_dir(Path::new("../../site"), &current_path)?;
    Ok(())
}

pub fn watch() -> io::Result<()> {
    Ok(())
}
