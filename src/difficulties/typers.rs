use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
    pub correct_index: i32,
}

#[derive(Debug, Deserialize)]
pub struct Questions {
    pub questions: Vec<Question>,
}
