use anyhow::Result;
use clap::StructOpt;

use std::io::stdin;
use rustytype::config::RustyTypeConfig;
use rustytype::RustyType;

fn main() -> Result<()> {
    let config = RustyTypeConfig::parse();

    let mut rustytype = RustyType::new(config)?;

    let stdin = stdin();

    loop {
        let stdin = stdin.lock();
        if let Ok((true, _)) = rustytype.test(stdin) {
            rustytype.restart()?;
        } else {
            break;
        }
    }
    Ok(())
}
