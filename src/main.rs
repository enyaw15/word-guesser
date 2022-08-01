use std::io;
use std::collections::HashSet;
fn main() {
    let words = ["hello", "world", "potato", "racecar", "tiger", "string", "rust" , "quiz"];
    let word = words[0];
    let word_len = word.len();
    let mut guesses:HashSet<char> = HashSet::new();
    let mut guess_count = 0;
    

    println!("Word length: {word_len}");
    loop {
        let mut victory = true;
        for letter in word.chars() {
            if guesses.contains(&letter) {
                print!("{letter}");
            } else {
                print!("_");
                victory = false;
            }
        }
        println!("");

        if victory {
            println!("You win!");
            return;
        }
    
        println!("Enter a letter to guess!");
        guess_count += 1;
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess:char = input.chars().next().unwrap();
        if guesses.contains(&guess)  {
            println!("You already guessed that letter");
        } else {
            guesses.insert(guess);
        }
        

        
    }
}
