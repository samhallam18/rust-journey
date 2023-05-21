// Hangman Game

use rand::Rng;

pub fn run_game() {
    let words = ["dog", "cat", "monkey", "donkey", "rhino", "capybara"];
    let random_num = rand::thread_rng().gen_range(0..=words.len() - 1);
    let chosen_word = words[random_num];
    println!("{} {random_num}", chosen_word);
}