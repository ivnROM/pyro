use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use clap::{Parser, error::ErrorKind};

mod locate;

#[derive(Parser)]
struct Cli{
    path: PathBuf,
    word: String,
}

fn main(){
    use locate::begin_search;

      // esto capta los argumentos, se peude hacer con std::env
      // tambien, la libreria lo simplifica
    let args = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => match e.kind() { ErrorKind::DisplayHelp => {
                 println!("{}", e);
                 std::process::exit(0);
             },
             ErrorKind::DisplayVersion => {
                 println!("{}", e);
                 std::process::exit(0);
             },
             _ => {
                 eprintln!("Error en la inserción de argumentos: {}", e);
                 std::process::exit(0);
             },
        },
    };
    
    let f = match File::open(&args.path){
        Ok(f) => f,
        Err(e) => panic!("Error: {}", e)
    };
    
    let bf = BufReader::new(f);
    let lowc_word = &args.word.to_lowercase(); 
    begin_search(bf, lowc_word);
}
