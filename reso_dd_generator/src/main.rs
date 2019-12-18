use serde_xml_rs::from_reader;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

mod case;
mod dtd;
mod generate;

fn main() -> Result<(), Box<dyn Error>> {
    let dtd_file = File::open("reso-ddwiki-export-2019-11-26-DDW17-v1.7.xml")?;

    fs::create_dir_all("reso_dd/src/generated")?;

    let path = Path::new("reso_dd/src/generated");

    let mut mod_file = File::create(path.join("mod.rs"))?;

    let dtd = from_reader::<File, dtd::Root>(dtd_file)?;

    dtd.generate(path, &mut mod_file)?;

    //    Command::new("sed")
    //        .arg("-i")
    //        .arg("")
    //        .arg("-e")
    //        .arg("s/[[:space:]]*//")
    //        .arg("reso_dd/src/generated.rs")
    //        .status()?;

    Command::new("cargo")
        .arg("fmt")
        .arg("-p")
        .arg("reso_dd")
        .status()?;

    Ok(())
}
