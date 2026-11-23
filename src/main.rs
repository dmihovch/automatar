use std::env;
use std::process::ExitCode;
use std::fs;
use std::error::Error;
mod mach;


fn run()-> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if !(2..=3).contains(&args.len()) {
        return Err("Usage: {} <formal_machine_description.fsa>, [input_string.is]".into());
    }
    let language: String = fs::read_to_string(&args[1])?;
    println!("Language: {}", language);
    
    let input = args.get(2)
        .map(|path| fs::read_to_string(path))
        .transpose()?;
    
    if let Some(content) = input {
        println!("Input: {}", content);
    } else {
        println!("No valid input received");
    }
    println!("{}", mach::generate_machine());
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
