use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Deserialize, Serialize, Debug)]
struct Phrase {
    #[serde(rename = "phraseId")]
    phrase_id: usize,
    phrase: String,
}

pub fn read_json() {
    let json = std::fs::read_to_string("./phrases.json").unwrap();
    let phrases = from_str::<Vec<Phrase>>(&json);
    println!("{:#?}", phrases);
}
