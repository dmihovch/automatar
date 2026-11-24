use std::env;
use std::process::ExitCode;
use std::fs;
use std::error::Error;
mod fsa;


fn run()-> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if !(2..=3).contains(&args.len()) {
       return Err("Usage: {} <formal_machine_description.fsa>, [input_string.is]".into());
    }
    let lang_file : String = fs::read_to_string(&args[1])?;
    println!("Language: {}", lang_file);
    
    //holy crust bro.
    //basically, map will read 'path' to string if 
    //args[2] exists, and then transpose
    //turns the Option of a Result into a Result of an Option
    let input = args.get(2)
        .map(|path| fs::read_to_string(path))
        .transpose()?;
    
    if let Some(content) = input {
        println!("Input: {}", content);
    } else {
        println!("No valid input received");
    }
    
    let fsa = fsa::FSA::new(&lang_file)?;

    for (state, symbol) in fsa.get_machine().keys() {
        println!("{state}, {symbol}");
    }

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
