use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Deserialize, Serialize, Debug)]
struct Phrase {
    #[serde(rename = "phraseId")]
    phrase_id: usize,
    phrase: String,
}

fn read_json() -> Result<Vec<Phrase>, serde_json::Error> {
    let json = std::fs::read_to_string("./phrases.json").unwrap();
    let phrases = from_str::<Vec<Phrase>>(&json);
    phrases
}

pub fn get_new_phrase() {
    let json = read_json();
    println!("{:#?}", json)
}
