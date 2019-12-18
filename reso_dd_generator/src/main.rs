use serde_xml_rs::from_reader;
use std::error::Error;
use std::fs::File;
use std::process::Command;

mod case;
mod dtd;
mod generate;

fn main() -> Result<(), Box<dyn Error>> {
    let dtd_file = File::open("reso-ddwiki-export-2019-11-26-DDW17-v1.7.xml")?;
    let mut lib_file = File::create("reso_dd/src/generated.rs")?;

    let dtd = from_reader::<File, dtd::Root>(dtd_file)?;

    dtd.generate(&mut lib_file)?;

    Command::new("sed")
        .arg("-i")
        .arg("")
        .arg("-e")
        .arg("s/[[:space:]]*//")
        .arg("reso_dd/src/generated.rs")
        .status()?;

    Command::new("cargo")
        .arg("fmt")
        .arg("-p")
        .arg("reso_dd")
        .status()?;

    Ok(())
}
