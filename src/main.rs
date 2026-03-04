use clap::Parser;
use std::io::{self, Read};
use std::process;

#[derive(Parser)]
#[command(name = "2md")]
#[command(about = "Convert HTML to Markdown", long_about = None)]
struct Args {
    #[arg(short, long, help = "Suppress error messages")]
    quiet: bool,

    #[arg(short, help = "Print version")]
    version: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.version {
        println!("2md v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let mut html = String::new();
    io::stdin().read_to_string(&mut html)?;

    if html.trim().is_empty() {
        if !args.quiet {
            eprintln!("Error: No input provided");
        }
        process::exit(2);
    }

    match html_to_markdown_rs::convert(&html, None) {
        Ok(markdown) => {
            print!("{}", markdown);
        }
        Err(e) => {
            if !args.quiet {
                eprintln!("Error: {}", e);
            }
            process::exit(1);
        }
    }

    Ok(())
}
