pub struct Identifier;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
    pub correct_index: i32,
}

#[derive(Debug, Deserialize)]
pub struct Responses {
    pub questions: Vec<Question>,
}

impl Identifier {
    pub fn identify_json() -> Responses {
        let data = include_str!("data/content.json");
        let responses: Responses = serde_json::from_str(data).unwrap();
        responses
    }
}
