use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;
use std::usize;

#[derive(Deserialize, Serialize, Debug)]
struct Phrase {
    #[serde(rename = "phraseId")]
    phrase_id: usize,
    phrase: String,
}

fn read_json() -> Result<String, serde_json::Error> {
    let json = fs::read_to_string("./phrases.json").unwrap();
    let phrases: Vec<Phrase> = from_str(&json)?;

    let amount_of_phrases: usize = phrases.len();
    let num = rand::thread_rng().gen_range(0..amount_of_phrases);
    let second_phrase = phrases
        .get(num) // access to the N element <-- needs to be dynamic
        .map(|phrase| phrase.phrase.clone())
        .unwrap_or_else(|| "There's no N phrase to display".to_string()); // Default value

    Ok(second_phrase)
}

pub fn get_new_phrase() {
    match read_json() {
        Ok(phrase) => println!("The Phrase is: {}", phrase),
        Err(err) => eprintln!("Error while reading JSON: {}", err),
    }
}
