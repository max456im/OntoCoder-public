```rust
//! ontoreg — реестр онтологических форм с CLA- и AENGA-поддержкой

use std::path::PathBuf;
use clap::{Command, Arg, SubCommand};
use crate::store::FormStore;
use crate::validator::validate_form_for_registry;

mod store;
mod metadata;
mod validator;
mod index;
mod exporter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("ontoreg")
        .version("2.0.0-aenga-public")
        .about("AENGA-guaranteed ontological form registry")
        .subcommand(
            SubCommand::with_name("register")
                .about("Register a compiled .ontobin form")
                .arg(Arg::new("file").required(true).index(1))
                .arg(Arg::new("jurisdiction").short('j').long("jurisdiction").takes_value(true).required(true))
        )
        .subcommand(
            SubCommand::with_name("search")
                .about("Search forms by jurisdiction, density, or tag")
                .arg(Arg::new("jurisdiction").short('j').long("jurisdiction"))
                .arg(Arg::new("min-density").short('d').long("min-density").takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("export")
                .about("Export form with full metadata")
                .arg(Arg::new("id").required(true).index(1))
        )
        .get_matches();

    let mut store = FormStore::new("./ontoreg_data")?;

    match matches.subcommand() {
        ("register", Some(sub_m)) => {
            let file = PathBuf::from(sub_m.value_of("file").unwrap());
            let jurisdiction = sub_m.value_of("jurisdiction").unwrap();
            store.register_form(&file, jurisdiction)?;
            println!("[ontoreg] Form registered under jurisdiction: {}", jurisdiction);
        }
        ("search", Some(sub_m)) => {
            let jur = sub_m.value_of("jurisdiction");
            let min_density = sub_m.value_of("min-density").and_then(|d| d.parse::<f32>().ok());
            let results = store.search(jur, min_density)?;
            for r in results {
                println!("ID: {} | Jur: {} | Density: {}", r.id, r.jurisdiction, r.density);
            }
        }
        ("export", Some(sub_m)) => {
            let id = sub_m.value_of("id").unwrap();
            let form = store.export_form(id)?;
            println!("{}", form);
        }
        _ => {}
    }

    Ok(())
}
```
