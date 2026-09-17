use dmg_maker::DmgMakerError;
use dmg_maker::{CreateOptions, create};
use std::path::PathBuf;

fn main() -> Result<(), DmgMakerError> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
        eprintln!("Usage: dmg_maker <json-path> <dmg-path>");
        return Ok(());
    }

    if args.len() == 1 && (args[0] == "--version" || args[0] == "-v") {
        eprintln!(env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if args.len() != 2 {
        return Err(DmgMakerError::General(
            "Usage: dmg_maker <json-path> <dmg-path>".to_string(),
        ));
    }

    let source = PathBuf::from(&args[0]);
    let target = PathBuf::from(&args[1]);

    create(CreateOptions {
        target,
        source: Some(source),
        basepath: None,
        specification: None,
    })
}
