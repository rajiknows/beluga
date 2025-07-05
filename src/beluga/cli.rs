use std::{
    fs, io,
    path::{self, Path},
};

use crate::beluga::{
    errors::BelugaError,
    utils::{BelugaError, copy_dir},
};
use anyhow::Result;

pub fn create(site_name: &String) -> io::Result<()> {
    println!("creating new project {site_name}");
    let current_path = std::env::current_dir()?;
    let current_path = current_path.join(site_name);
    copy_dir(Path::new("../../site"), &current_path)?;

    // show the nest steps
    println!(
        r#"
        next steps:
        cd {}\n

        run the project:
        beluga watch

        edit the files in src/ to see the changes
        "#,
        site_name
    );

    Ok(())
}

pub fn watch(project_name: &str) -> Result<(), BelugaError> {
    // check if there is a project file inside
    if !is_beluga_project(project_name) {
        return Err(BelugaError::ProjectNotInitialised);
    }

    // if there is dist directoru go ahead and deploy it to localhost 42069
    if is_dist_ready(project_name) {
        return serve_site(project_name);
    }
    // fallback if not ready

    // build the project
    build_site(project_name)?;
    serve_site(project_name)?;
    Ok(())
}

pub fn is_dist_ready(path: &str) -> bool {
    let path = Path::new(path);
    path.join("dist").exists()
}

pub fn is_beluga_project(path: &str) -> bool {
    let path = Path::new(path);
    if !path.exists() || !path.is_dir() {
        return false;
    }

    path.join("beluga.yml").exists()
}

fn serve_site(project_name: &str) -> Result<(), BelugaError> {
    Ok(())
}

fn build_site(project_name: &str) -> Result<(), BelugaError> {
    Ok(())
}
