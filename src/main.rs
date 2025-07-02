mod beluga;

use std::io;

use crate::beluga::cli as luga;
use clap::CommandFactory;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Beluga")]
#[command(about = "A cute squishy static site generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new site
    Init {
        #[arg(default_value = "my-site")]
        name: String,
    },
    /// Watch for changes and rebuild
    Watch,
    /// Show help
    Help,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { name } => {
            println!("Initializing site: {}", name);
            luga::create(name);
            Ok(())
        }
        Commands::Watch => {
            println!("Watching for changes...");
            luga::watch()
        }
        Commands::Help => {
            Cli::command().print_help().unwrap();
            println!();
            Ok(())
        }
    }
}

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
//     println!("Serving on http://127.0.0.1:8080");
//     // let textnode = TextNode::new("this is a text node".to_string(), TextType::Bold, None);
//     // let html = textnode.to_html_node().to_string();
//     // println!("{html}");

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//         handle_connection(stream);
//     }
// }

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();

//     let request = String::from_utf8_lossy(&buffer);
//     let path = request
//         .lines()
//         .next()
//         .unwrap()
//         .split_whitespace()
//         .nth(1)
//         .unwrap();

//     if path != "/" {
//         let response = b"HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";
//         stream.write_all(response).unwrap();
//         return;
//     }

//     let contents = fs::read_to_string("./public/index.html").unwrap();

//     let response = format!(
//         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
//         contents.len(),
//         contents
//     );

//     stream.write_all(response.as_bytes()).unwrap();
// }
