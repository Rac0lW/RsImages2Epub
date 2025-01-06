use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use epub_builder::{EpubBuilder, EpubContent, EpubVersion, ReferenceType, ZipLibrary};

fn run(image_folder: &str) -> epub_builder::Result<Vec<u8>> {
    let mut output = Vec::<u8>::new();

    // 先创建 ZipLibrary 实例
    let zip_library = ZipLibrary::new()?;
    // 然后将其传递给 EpubBuilder
    let mut builder = EpubBuilder::new(zip_library).unwrap();
    
    builder.metadata("author", "unknown")?;
    builder.metadata("title", "afterglow")?;
    builder.epub_version(EpubVersion::V30);

    // Read images from the folder and add them to the EPUB
    let paths = fs::read_dir(image_folder)?;
    for (i, path) in paths.enumerate() {
        let path = path?.path();
        if path.is_file() {
            let image_data = fs::read(&path)?;
            let image_name = path.file_name().unwrap().to_str().unwrap();

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
    let image_folder = r"C:\\Users\\racol\\Desktop\\New folder\\image\\flowers";

    // Generate EPUB
    let output = run(image_folder).expect("Unable to create an epub document");

    // Define the path to the desktop
    let desktop_path = Path::new(r#"C:\Users\racol\Desktop\1.epub"#);

    // Create and write to the file
    let mut file = File::create(desktop_path).expect("Error01");
    file.write_all(&output)?;

    Ok(())
}