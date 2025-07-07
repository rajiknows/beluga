use crate::beluga::{
    engine::{HtmlNode, process_file_to_vec_of_nodes},
    errors::BelugaError,
    utils::copy_dir,
};
use anyhow::Result;
use notify::{RecursiveMode, Watcher};
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::mpsc::channel,
};
use tera::{Context, Tera};
use tiny_http::{Response, Server};
use walkdir::WalkDir;

pub fn create(site_name: &String) -> io::Result<()> {
    println!("creating new project {site_name}");
    let current_path = std::env::current_dir()?;
    let current_path = current_path.join(site_name);
    fs::create_dir(current_path.clone())?;
    copy_dir(Path::new("site"), &current_path)?;

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
    if !is_beluga_project(project_name) {
        return Err(BelugaError::ProjectNotInitialised);
    }

    build_site(project_name)?;
    serve_site(project_name)?;

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx).unwrap();
    watcher
        .watch(
            Path::new(project_name).join("src").as_path(),
            RecursiveMode::Recursive,
        )
        .unwrap();

    loop {
        match rx.recv() {
            Ok(_) => {
                println!("Rebuilding site...");
                build_site(project_name)?;
                serve_site(project_name)?;
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

pub fn is_dist_ready(path: &str) -> bool {
    let path = Path::new(path);
    path.join("dist").exists()
}

pub fn is_beluga_project(path: &str) -> bool {
    let path = Path::new(path);
    path.join("beluga.yml").exists()
}

fn serve_site(project_name: &str) -> Result<(), BelugaError> {
    let server = Server::http("127.0.0.1:42069").unwrap();
    println!("Server started at http://127.0.0.1:42069");

    for request in server.incoming_requests() {
        let mut path = PathBuf::from(project_name).join("dist");
        let requested_path = request.url().trim_start_matches('/');

        if requested_path.is_empty() {
            path.push("index.html");
        } else {
            path.push(requested_path);
        }

        if path.is_dir() {
            path.push("index.html");
        }

        let response = if path.exists() {
            let content = fs::read_to_string(&path).unwrap();
            Response::from_string(content).with_header(
                "Content-Type: text/html"
                    .parse::<tiny_http::Header>()
                    .unwrap(),
            )
        } else {
            Response::from_string("404 Not Found").with_status_code(404)
        };

        request.respond(response).unwrap();
    }
    Ok(())
}

fn build_site(project_name: &str) -> Result<(), BelugaError> {
    let base_path = Path::new(project_name);
    let src_path = base_path.join("src");
    let dist_path = base_path.join("dist");
    let template_path = base_path.join("templates");

    if !dist_path.exists() {
        fs::create_dir_all(&dist_path).unwrap();
    }

    let tera = Tera::new(&format!("{}/**/*.html", template_path.to_str().unwrap())).unwrap();

    for entry in WalkDir::new(&src_path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().unwrap_or_default() == "md" {
            let file = fs::File::open(path).unwrap();
            let nodes = process_file_to_vec_of_nodes(file);
            let html_content = nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<String>();

            let mut context = Context::new();
            context.insert("content", &html_content);

            let template_name = {
                let relative_path = path.strip_prefix(&src_path).unwrap();
                if relative_path.ancestors().count() > 2 {
                    // In a subdirectory, use the directory name for the template.
                    relative_path
                        .parent()
                        .unwrap() // Safe to unwrap here
                        .file_name()
                        .and_then(|s| s.to_str())
                        .map(|s| format!("{}.html", s))
                        .unwrap_or_else(|| "index.html".to_string())
                } else {
                    // In src root, use the file stem for the template.
                    relative_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| format!("{}.html", s))
                        .unwrap_or_else(|| "index.html".to_string())
                }
            };

            let rendered_html = tera.render(&template_name, &context).unwrap();

            let relative_path = path.strip_prefix(&src_path).unwrap();
            let dest_path = dist_path.join(relative_path).with_extension("html");

            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }

            let mut file = fs::File::create(&dest_path).unwrap();
            file.write_all(rendered_html.as_bytes()).unwrap();
        } else if path.is_dir() {
            // if it is a folder ...
            for entry in WalkDir::new(path) {}
        }
    }
    Ok(())
}
