```rust
//! ontoc — компилятор онтологических форм OntoCoder Public 2.0
//! Обязательно: AENGA + SGCL + CLA

use std::path::PathBuf;
use clap::{Command, Arg};
use crate::parser::parse_onto_file;
use crate::validator::validate_and_compile;

mod parser;
mod validator;
mod aenga_check;
mod sgcl_check;
mod cla_check;
mod codegen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("ontoc")
        .version("2.0.0-aenga-public")
        .about("AENGA-guaranteed ontological compiler")
        .arg(Arg::new("input")
            .help("Input .onto file")
            .required(true)
            .index(1))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .help("Output .ontobin file")
            .required(true)
            .takes_value(true))
        .get_matches();

    let input_path = PathBuf::from(matches.value_of("input").unwrap());
    let output_path = PathBuf::from(matches.value_of("output").unwrap());

    println!("[ontoc] Compiling {} → {}", input_path.display(), output_path.display());

    let form = parse_onto_file(&input_path)?;
    validate_and_compile(&form, &output_path)?;

    println!("[ontoc] SUCCESS: Ontoform compiled with AENGA, SGCL, CLA compliance.");
    Ok(())
}
```