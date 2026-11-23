fn run()-> Result<(), Box< dyn std::error::Error>> {
    rand.
}

fn main() {
    match run() {
        Ok(()) => println!("Ok!" ),
        Err(error) => eprintln!("Error: {}", error),
    }
}
