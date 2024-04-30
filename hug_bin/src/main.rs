use std::{fs::OpenOptions, io::Read};

use clap::{crate_authors, crate_description, crate_name, crate_version, PossibleValue};
use hug_interpreter::vm::HugVM;

fn app() -> clap::App<'static> {
    clap::App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(clap::Arg::new("command")
            .index(1)
            .required(true)
            .possible_values([
                PossibleValue::new("run").alias("r").help("Transpile and run"),
                PossibleValue::new("transpile").alias("t").help("Convert a .hug file into a list of instructions"),
                PossibleValue::new("compile").alias("c").help("Convert a .hug file into a distributable application!"),
            ]))
        .arg(clap::Arg::new("input_file")
            .index(2)
            .required(false)
            .help("If you're not using a project.hug file to describe your project's layout and dependencies, you can directly use a file through this parameter."))
}

fn main() {
    let app = app().get_matches();

    match app.value_of("command").unwrap() {
        "r" | "run" => {
            let mut vm = HugVM::new(app.value_of("input_file").unwrap_or_else(|| todo!())); // TODO: Read project.hug
            vm.run();
        }
        "t" | "transpile" => {}
        "c" | "compile" => {
            let file_name = app.value_of("input_file").unwrap_or_else(|| todo!()); // TODO: Read project.hug
            let mut file = OpenOptions::new()
                .read(true)
                .open(file_name)
                .expect(format!("Could not open file {}!", file_name).as_str());

            let mut buffer = String::new();
            file.read_to_string(&mut buffer)
                .expect("Could not read file!");

            // hug_compiler::compile(buffer);
            todo!()
        }
        _ => unreachable!(),
    }
}

#[test]
fn check_app() {
    app().debug_assert()
}
