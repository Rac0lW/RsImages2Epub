

use std::error::Error;
use std::fs::File;
use std::io::Write;
use ansi_term::Colour;
use rs_image_2_epub::{get_desktop_path, run};
use log::info;

use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    #[arg(short, long)]
    address: String
}


fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("start up");
    // Path to the folder containing images
    let args = Args::parse();

    let input = args.address;

    let input = input.trim();

    let image_folder = input;

    // Generate EPUB
    let output = run(image_folder).expect("Unable to create an epub document");

    // Locate the desktop's path
    let desktop_path = get_desktop_path();

    // Create and write to the file
    let mut file = File::create(desktop_path.as_str()).expect("Error01");
    file.write_all(&output)?;

    println!(
        "[{}] Work is down, see the file's location {}",
        Colour::Green.paint("Success"),
        desktop_path.as_str()
    );

    Ok(())

    
}
