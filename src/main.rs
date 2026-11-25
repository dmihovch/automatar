use std::env;
use std::process::ExitCode;
use std::fs;
use std::error::Error;
mod fsa;


fn run()-> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
       return Err("Usage: {} <formal_machine_description.fsa>, <input_string.is>".into());
    }
    let lang_file : String = fs::read_to_string(&args[1])?;
    println!("Language: {}", lang_file);
    let input: String = fs::read_to_string(&args[2])?;
    let fsa = fsa::FSA::new(&lang_file)?;
    println!("DEBUG:");
    fsa.print_machine();
    println!("Transitions:");
    for (state, symbol) in fsa.get_machine().keys() {
        println!("{state}, {symbol}");
    }
    fsa.evaluate_string(&input);
    return Ok(());
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => {
            println!("Ok!");
            return ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            return ExitCode::FAILURE;
        }    
    }
}
