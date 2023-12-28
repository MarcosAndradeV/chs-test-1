use chsvm::{load_file, exepitons::GenericError, compile_chs_file, run_file};
use clap::{Arg, Command};

fn main() -> Result<(), GenericError>{
    // Basic CLI just for tests.
    let cmd = Command::new("chsvm")
        .about("...")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Marcos V. Andrade Almeida")
        .subcommand(
            Command::new("run")
                .about("...")
                .arg(Arg::new("filename").value_name("FILE").num_args(1)),

        )
        .get_matches();

    match cmd.subcommand() {
        Some(("run", file_matches)) => {
            if file_matches.contains_id("filename") {
                let filename = file_matches
                    .get_one::<String>("filename")
                    .expect("contains_id");
                let res = load_file(filename)?;
                let res2 = compile_chs_file(res)?;
                run_file(res2)?;
                return Ok(());
            }
            println!("File not provided.");
            println!("Usage: chsvm <file.chs>");
            return Ok(());

        }
        _ => unreachable!()
    }
}