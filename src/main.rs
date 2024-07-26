use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{Parser, error::ErrorKind};
use colored::*;

#[derive(Parser)]
struct Cli{
    path: PathBuf,
    word: String,
}

fn print_color(word: &str, line: &str){
    
   let word_index = line
       .to_lowercase()
       .find(word)
       .unwrap();

   let word_end = (word_index + word.chars().count()) as usize;
   
   print!("{}", line[..word_index].to_string());
   print!("{}", line[word_index..word_end].yellow());
   println!("{}", line[word_end..].to_string());
}

fn main(){

      // esto capta los argumentos, se peude hacer con std::env tambien, la libreria lo simplifica
    let args = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => match e.kind() {
             ErrorKind::DisplayHelp => {
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
    let mut idx = 1;
    for line in bf.lines() {
        let line = line.unwrap();
        if line
            .to_lowercase()
            .contains(lowc_word) {
            print!("[{}] ", idx);
            print_color(lowc_word, &line);
        }
        idx += 1;
    }
}
