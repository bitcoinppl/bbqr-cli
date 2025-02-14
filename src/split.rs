use std::{path::Path, time::Duration};

use crate::SplitArgs;
use bbqr::{
    file_type::FileType,
    qr::Version,
    split::{Split, SplitOptions},
};
use color_eyre::owo_colors::OwoColorize as _;
use eyre::{Context as _, Result};
use fast_qr::{
    convert::{image::ImageBuilder, Builder as _},
    QRBuilder,
};

pub fn run(args: SplitArgs) -> Result<()> {
    let mut split_options = SplitOptions {
        min_split_number: args.min_splits,
        max_split_number: args.max_splits,
        encoding: bbqr::encode::Encoding::Zlib,
        ..Default::default()
    };

    split_options.min_version = Version::try_from(args.min_version)?;
    split_options.max_version = Version::try_from(args.max_version)?;

    // if version is provided, use it
    if let Some(version) = args.version {
        if (1..=40).contains(&version) {
            split_options.min_version = Version::try_from(version)?;
            split_options.max_version = Version::try_from(version)?;
        }
    }

    let data = read_file_if_exists(&args.input)?.unwrap_or(args.input.as_bytes().to_vec());
    let split = Split::try_from_data(&data, FileType::UnicodeText, split_options)?;

    // output to file
    if let Some(output_path) = &args.output {
        return output_to_file(Path::new(output_path), &split);
    }

    // output to terminal
    output_to_terminal(&split)
}

fn output_to_file(output_path: &Path, split: &Split) -> Result<()> {
    std::fs::create_dir_all(output_path).wrap_err("Failed to create output directory")?;

    for (i, part) in split.parts.iter().enumerate() {
        // Create a QR code
        let qr = QRBuilder::new(part.as_bytes())
            .version(split.version.into())
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

fn output_to_terminal(split: &Split) -> Result<()> {
    let total_parts = split.parts.len();

    let version_number = split.version as u16 + 1;
    let total_width = ((version_number * 4) + 17) + (4 * 2);
    let total_width = total_width as usize;

    loop {
        for (i, part) in split.parts.iter().enumerate() {
            // Create a QR code
            let qr = QRBuilder::new(part.as_bytes())
                .mode(fast_qr::Mode::Alphanumeric)
                .ecl(fast_qr::ECL::L)
                .version(split.version.into())
                .build()
                .wrap_err("Failed to create QR code")?;

            let unicode = qr.to_str();

            // Count number of lines in QR code
            let lines = unicode.lines().count();

            // Move cursor up by number of lines in QR
            println!("{}", unicode);

            let part_number_string = format!("Part {} of {}", i + 1, total_parts);
            let spaces_for_centering = (total_width / 2) - part_number_string.len() / 2;

            println!(
                "\n{}{}",
                " ".repeat(spaces_for_centering),
                part_number_string.bold().green()
            );

            let msg = "Press Ctrl+C to exit";
            let spaces_for_centering = (total_width / 2) - msg.len() / 2;

            println!("{}{}", " ".repeat(spaces_for_centering), msg.yellow());
            print!("\x1B[{}A", lines + 3);

            std::thread::sleep(Duration::from_millis(200));
        }
    }
}

fn read_file_if_exists(path: &str) -> Result<Option<Vec<u8>>> {
    let path = Path::new(path);
    if !path.exists() {
        return Ok(None);
    }

    let contents = std::fs::read(path).wrap_err("Failed to read file")?;
    Ok(Some(contents))
}
