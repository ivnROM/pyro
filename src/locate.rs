use std::io::{BufReader, BufRead, Read};
use colored::*;

fn print_color(word_to_search: &str, line: &str){
    for word in line.split_whitespace() {
        if !word.to_lowercase().contains(&word_to_search) {
            print!("{} ", word);
            continue;
        }

        let word_index = word.to_lowercase()
                            .find(&word_to_search)
                            .unwrap();

        let word_end = (word_index + word_to_search.chars().count()) as usize;
        print!("{}", word[..word_index].to_string());
        print!("{}", word[word_index..word_end].yellow().bold());
        print!("{} ", word[word_end..].to_string());
    }
    print!("\n");
}

pub fn begin_search<R: Read>(bf: BufReader<R>, word: &String) {
    let mut idx = 1;
    
    for line in bf.lines() {
        let line = line.unwrap();
        if line.to_lowercase().contains(word) {
            print!("[{}] ", idx);
            print_color(word, &line);
         }
        idx += 1;
    }
}
