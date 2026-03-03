use std::io::{self, BufRead};

use bbqr::{file_type::FileType, join::Joined};
use eyre::Result;

use crate::MergeArgs;

pub fn run(args: MergeArgs) -> Result<()> {
    let parts = match args.input {
        Some(input) => vec![input],
        None => {
            let stdin = io::stdin();
            stdin
                .lock()
                .lines()
                .collect::<Result<Vec<_>, _>>()?
        }
    };

    let joined = Joined::try_from_parts(parts)?;

    match joined.file_type {
        FileType::UnicodeText => {
            let text = String::from_utf8(joined.data)?;
            println!("{text}");
        }
        _ => {
            // for binary types, output hex
            let hex = joined
                .data
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>();
            println!("{hex}");
        }
    }

    Ok(())
}
