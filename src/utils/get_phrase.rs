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

pub fn get_new_phrase() -> Result<String, serde_json::Error> {
    let json = fs::read_to_string("./phrases.json").unwrap();
    let phrases: Vec<Phrase> = from_str(&json)?;

    let amount_of_phrases: usize = phrases.len();
    let num = rand::thread_rng().gen_range(0..amount_of_phrases);
    let selected_phrase = phrases
        .get(num)
        .map(|phrase| phrase.phrase.clone())
        .unwrap_or_else(|| "There's no N phrase to display".to_string()); // Default value

    Ok(selected_phrase.to_string())
}

