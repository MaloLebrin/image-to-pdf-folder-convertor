# Image to PDF Folder Converter

**A command-line tool written in Rust for converting images to compressed PDF files.**

This program recursively traverses a folder of images (JPEG, PNG), automatically compresses each image, and converts it into an individual PDF file, ensuring each PDF is less than 1 MB.

---

## 📌 Features

- ✅ **Automatic conversion** : Converts JPEG and PNG images into individual PDF files
- ✅ **Smart compression** : Automatic compression until the PDF is less than 1 MB
- ✅ **Recursive traversal** : Processes all image files in a folder and its subfolders
- ✅ **Unique filenames** : Uses timestamps to avoid naming conflicts
- ✅ **Detailed logs** : Displays progress and information for each conversion
- ✅ **Robust error handling** : Proper error handling with clear messages

---

## 🛠 Prerequisites

- [Rust](https://www.rust-lang.org/) (version 1.70 or higher)
- `cargo` (included with Rust installation)

### Verify Rust Installation

```bash
rustc --version
cargo --version
```

If Rust is not installed, follow the instructions on [rustup.rs](https://rustup.rs/).

---

## 🚀 Installation

### 1. Clone the repository

```bash
git clone https://github.com/ton-utilisateur/image-to-pdf-folder-convertor.git
cd image-to-pdf-folder-convertor
```

### 2. Build the project

For debug build (faster, for development):

```bash
cargo build
```

For optimized release build (recommended for production):

```bash
cargo build --release
```

The executable will be available in:
- Debug mode: `target/debug/image-to-pdf-folder-convertor`
- Release mode: `target/release/image-to-pdf-folder-convertor`

---

## 📖 Usage

### Basic syntax

```bash
./target/release/image-to-pdf-folder-convertor -i <input_folder> -o <output_folder>
```

### Options

| Option | Short | Description | Required | Default |
|--------|-------|-------------|----------|---------|
| `--input-dir` | `-i` | Path to the folder containing images | ✅ Yes | - |
| `--output-dir` | `-o` | Path to the output folder for PDFs | ❌ No | `output` |

### Usage examples

#### Example 1: Basic conversion

```bash
./target/release/image-to-pdf-folder-convertor -i ~/Downloads/images -o ~/Downloads/converted_pdfs
```

#### Example 2: Using default output folder

```bash
./target/release/image-to-pdf-folder-convertor -i ./photos
# PDFs will be created in the ./output folder
```

#### Example 3: Using relative paths

```bash
./target/release/image-to-pdf-folder-convertor -i ./input_images -o ./pdf_output
```

#### Example 4: Using absolute paths

```bash
./target/release/image-to-pdf-folder-convertor -i /Users/malolebrin/Pictures -o /Users/malolebrin/Documents/PDFs
```

### Output filename format

Generated PDF files follow the format:
```
<original_image_name>_<unix_timestamp>.pdf
```

Example: If you have an image `vacation.jpg`, the generated PDF might be `vacation_1704067200.pdf`

**Note**: Spaces in original filenames are replaced with underscores (`_`).

---

## 🔧 Technical Details

### Conversion process

1. **Recursive traversal** : The program recursively traverses the input folder looking for image files
2. **Format detection** : Only files with `.jpg`, `.jpeg`, and `.png` extensions are processed
3. **Image loading** : Each image is loaded into memory
4. **Compression** : The image is compressed as JPEG with an initial quality of 85%, then reduced in 5% increments until it reaches a size below 1 MB or a minimum quality of 10%
5. **PDF creation** : A PDF document is created with:
   - A4 page format (595x842 points)
   - Image resized to 75% of its original size
   - DCT (JPEG) compression for the image
6. **Saving** : The PDF is saved to the output folder with a unique name

### Technical specifications

- **PDF format** : Version 1.5
- **Target size** : 1 MB maximum per PDF
- **Initial JPEG quality** : 85%
- **Minimum JPEG quality** : 10%
- **Quality reduction** : In 5% increments
- **Color space** : DeviceRGB (8 bits per component)
- **Page size** : A4 (595x842 points)

---

## 📁 Project Structure

```
image-to-pdf-folder-convertor/
├── Cargo.toml          # Project configuration and dependencies
├── Cargo.lock          # Locked dependency versions
├── README.md           # This documentation
└── src/
    └── main.rs         # Main source code
```

### Dependencies

- **`image`** (0.24) : Library for image processing
- **`lopdf`** (0.27) : Library for PDF creation and manipulation
- **`walkdir`** (2) : Recursive directory traversal
- **`clap`** (4.0) : Command-line argument parsing

---

## 🐛 Troubleshooting

### Issue: "No such file or directory"

**Cause**: The input or output folder does not exist.

**Solution**: 
- Verify that the input folder path is correct
- The output folder will be created automatically if it doesn't exist

### Issue: "Permission denied"

**Cause**: Insufficient permissions to read the input folder or write to the output folder.

**Solution**: 
- Check folder permissions
- On Linux/Mac, you might need to use `sudo` (not recommended) or modify permissions

### Issue: No images are converted

**Cause**: No files with `.jpg`, `.jpeg`, or `.png` extensions were found.

**Solution**: 
- Verify that your images have one of these extensions (lowercase or uppercase)
- Verify that the input folder path is correct

### Issue: PDF is too large

**Cause**: Even with minimum quality, the image is too large.

**Solution**: 
- The program tries to compress down to 10% quality, but some very large images may still exceed 1 MB
- Consider resizing the image before conversion

### Issue: Compilation error

**Cause**: Outdated Rust version or missing dependencies.

**Solution**: 
```bash
rustup update
cargo clean
cargo build --release
```

---

## 📝 Example Output

When running, you will see messages like:

```
Traitement de : "/Users/malolebrin/Downloads/images/photo1.jpg"
PDF créé : "/Users/malolebrin/Downloads/converted_pdfs/photo1_1704067200.pdf" (taille : 856432 octets)
Traitement de : "/Users/malolebrin/Downloads/images/photo2.png"
PDF créé : "/Users/malolebrin/Downloads/converted_pdfs/photo2_1704067201.pdf" (taille : 923145 octets)
3 images traitées.
```

*Note: The output messages are currently in French. The messages indicate: "Processing:", "PDF created:", "size:", and "images processed."*

Wait, I notice the output messages are still in French. Let me check the source code to see what the actual output messages are, or I should update the example to match what would be shown in English. Actually, looking at the code, the messages are in French. For the documentation, I should show what the output would look like, but since the code outputs in French, I should either note that or show the actual output. Let me update this section to be more accurate.

Actually, let me check the main.rs file to see the exact output messages.
<｜tool▁calls▁begin｜><｜tool▁call▁begin｜>
read_file
