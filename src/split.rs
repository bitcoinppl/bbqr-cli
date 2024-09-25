use crate::SplitArgs;
use bbqr::{
    file_type::FileType,
    qr::Version,
    split::{Split, SplitOptions},
};
use eyre::{Context as _, Result};
use fast_qr::{
    convert::{image::ImageBuilder, Builder as _},
    QRBuilder,
};

pub fn run(args: SplitArgs) -> Result<()> {
    let mut split_options = SplitOptions {
        min_split_number: args.min_splits,
        max_split_number: args.max_splits,
        ..Default::default()
    };

    if let Some(version) = args.version {
        if (1..=40).contains(&version) {
            split_options.min_version = Version::try_from(version)?;
            split_options.max_version = Version::try_from(version)?;
        }
    }

    let data = read_file_if_exists(&args.input)?.unwrap_or(args.input.as_bytes().to_vec());

    let split = Split::try_from_data(&data, FileType::UnicodeText, split_options)?;

    let output_path = std::path::Path::new(&args.output);
    std::fs::create_dir_all(output_path).wrap_err("Failed to create output directory")?;

    for (i, part) in split.parts.iter().enumerate() {
        // Create a QR code
        let qr = QRBuilder::new(part.as_bytes())
            .build()
            .wrap_err("Failed to create QR code")?;

        let path = output_path.join(format!("qrcode_{}.png", i + 1));

        // Convert QR code to an image
        ImageBuilder::default()
            .shape(fast_qr::convert::Shape::Square)
            .to_file(&qr, path.as_os_str().to_str().expect("valid path"))
            .wrap_err("Failed to to save image of QR code")?;
    }

    Ok(())
}

fn read_file_if_exists(path: &str) -> Result<Option<Vec<u8>>> {
    let path = std::path::Path::new(path);
    if !path.exists() {
        return Ok(None);
    }

    let contents = std::fs::read(path).wrap_err("Failed to read file")?;
    Ok(Some(contents))
}
