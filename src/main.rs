use image::{io::Reader as ImageReader, ImageFormat};
use lopdf::{Document, Object, Dictionary, Stream};
use std::path::PathBuf;
use std::fs;
use std::io::Cursor;
use walkdir::WalkDir;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    input_dir: String,
    #[clap(short, long, default_value = "output")]
    output_dir: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    fs::create_dir_all(&args.output_dir)?;

    for entry in WalkDir::new(&args.input_dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ["jpg", "jpeg", "png"].contains(&ext.to_string_lossy().to_lowercase().as_str()) {
                    let output_path = PathBuf::from(&args.output_dir)
                        .join(path.file_stem().unwrap())
                        .with_extension("pdf");

                    // 1. Charger et compresser l'image
                    let img = ImageReader::open(path)?.decode()?;
                    let mut compressed_img = Vec::new();
                    let mut quality = 85u8;
                    let target_size = 1_000_000; // 1 Mo

                    loop {
                        compressed_img.clear();
                        let mut cursor = Cursor::new(&mut compressed_img);
                        img.write_to(&mut cursor, ImageFormat::Jpeg)?;
                        if compressed_img.len() < target_size || quality < 10 {
                            break;
                        }
                        quality -= 5;
                    }

                    // 2. Cloner compressed_img pour l'affichage
                    let compressed_img_for_pdf = compressed_img.clone();

                    // 3. Créer un document PDF avec lopdf
                    let mut doc = Document::with_version("1.5");

                    // 4. Créer le dictionnaire pour l'image
                    let mut img_dict = Dictionary::new();
                    img_dict.set("Type", Object::Name(b"XObject".to_vec()));
                    img_dict.set("Subtype", Object::Name(b"Image".to_vec()));
                    img_dict.set("Width", Object::Integer(img.width() as i64));
                    img_dict.set("Height", Object::Integer(img.height() as i64));
                    img_dict.set("ColorSpace", Object::Name(b"DeviceRGB".to_vec()));
                    img_dict.set("BitsPerComponent", Object::Integer(8));
                    img_dict.set("Filter", Object::Name(b"DCTDecode".to_vec()));

                    let img_object_id = doc.add_object(Stream::new(img_dict, compressed_img_for_pdf));

                    let pages_id = doc.new_object_id();
                    let catalog_id = doc.new_object_id();
                    let page_id = doc.new_object_id();
                    let content_id = doc.new_object_id();

                    // 5. Créer le contenu de la page
                    let content = format!(
                        "q\n{} 0 0 {} 0 0 cm\n/Im0 Do\nQ",
                        img.width() as f64 * 0.75,  // Échelle pour s'adapter à la page
                        img.height() as f64 * 0.75
                    );
                    doc.objects.insert(
                        content_id,
                        Object::Stream(Stream::new(Dictionary::new(), content.into_bytes())),
                    );

                    // 6. Créer les dictionnaires pour la page et le catalogue
                    let mut pages_dict = Dictionary::new();
                    pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
                    pages_dict.set("Kids", Object::Array(vec![Object::Reference(page_id)]));
                    pages_dict.set("Count", Object::Integer(1));
                    doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

                    let mut page_dict = Dictionary::new();
                    page_dict.set("Type", Object::Name(b"Page".to_vec()));
                    page_dict.set("Parent", Object::Reference(pages_id));
                    page_dict.set(
                        "MediaBox",
                        Object::Array(vec![
                            Object::Integer(0),
                            Object::Integer(0),
                            Object::Integer(595), // A4 width in points
                            Object::Integer(842), // A4 height in points
                        ]),
                    );
                    page_dict.set("Contents", Object::Reference(content_id));

                    let mut resources_dict = Dictionary::new();
                    let mut xobject_dict = Dictionary::new();
                    xobject_dict.set("Im0", Object::Reference(img_object_id));
                    resources_dict.set("XObject", Object::Dictionary(xobject_dict));
                    page_dict.set("Resources", Object::Dictionary(resources_dict));

                    doc.objects.insert(page_id, Object::Dictionary(page_dict));

                    let mut catalog_dict = Dictionary::new();
                    catalog_dict.set("Type", Object::Name(b"Catalog".to_vec()));
                    catalog_dict.set("Pages", Object::Reference(pages_id));
                    doc.objects.insert(catalog_id, Object::Dictionary(catalog_dict));

                    // 7. Définir le catalogue comme racine
                    doc.trailer.set("Root", Object::Reference(catalog_id));

                    // 8. Sauvegarder le PDF
                    doc.save(&output_path)?;

                    println!("PDF créé : {:?} (taille : {} octets)", output_path, compressed_img.len());
                }
            }
        }
    }
    Ok(())
}
