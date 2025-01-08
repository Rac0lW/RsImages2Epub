use core::panic;
use directories::UserDirs;
use epub_builder::{EpubBuilder, EpubContent, EpubVersion, ReferenceType, ZipLibrary};
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
// use std::path::Path;

fn run(image_folder: &str) -> epub_builder::Result<Vec<u8>> {
    let mut output = Vec::<u8>::new();

    let zip_library = ZipLibrary::new()?;

    let mut builder = EpubBuilder::new(zip_library).unwrap();

    builder.metadata("author", "unknown")?;
    builder.metadata("title", "unknown")?;
    builder.epub_version(EpubVersion::V30);

    // Read images from the folder and add them to the EPUB
    let paths = fs::read_dir(image_folder)?;

    let mut cover_index_flag: i32 = 1;

    for (i, path) in paths.enumerate() {
        let path = path?.path();
        if path.is_file() {
            let image_data = fs::read(&path)?;
            let image_name = path.file_name().unwrap().to_str().unwrap();
            if cover_index_flag == 1 {
                builder.add_cover_image(image_name, &*image_data, "image/png")?;
                cover_index_flag += 1;
            }
            // Add image as a resource
            builder.add_resource(image_name, &*image_data, "image/png")?;

            // Create XHTML content for each image
            let xhtml_content = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
                <html xmlns="http://www.w3.org/1999/xhtml">
                <head><title>Image {}</title></head>
                <body><img src="{}" alt="Image {}" /></body>
                </html>"#,
                i + 1,
                image_name,
                i + 1
            );

            builder.add_content(
                EpubContent::new(format!("image_{}.xhtml", i + 1), xhtml_content.as_bytes())
                    .title(format!("Image {}", i + 1))
                    .reftype(ReferenceType::Text),
            )?;
        }
    }

    builder.generate(&mut output)?;

    Ok(output)
}

fn main() -> io::Result<()> {
    // Path to the folder containing images
    let args: Vec<String> = env::args().collect();

    let input: String;

    if args.len() > 1 {
        input = args[1].clone();
    } else {
        panic!("New a folder path of your images.");
    }

    let input = input.trim();

    let image_folder = input;

    // Generate EPUB
    let output = run(image_folder).expect("Unable to create an epub document");

    // Locate the desktop's path
    let desktop_path;

    if let Some(user_dir) = UserDirs::new() {
        if let Some(path) = user_dir.desktop_dir() {
            let name = String::from("\\new.epub");

            desktop_path = String::from(path.to_str().unwrap()) + name.as_str();
        } else {
            panic!("Could not find the desktop directory.");
        }
    } else {
        panic!("Could not determine user directories.");
    }

    println!("{:?}", desktop_path);
    // Create and write to the file
    let mut file = File::create(desktop_path.as_str()).expect("Error01");
    file.write_all(&output)?;

    Ok(())
}
