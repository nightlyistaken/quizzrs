pub struct HardDifficulty;
use crate::difficulties::typers::Questions;

impl HardDifficulty {
    pub fn get_questions() -> Questions {
        let data = include_str!("../data/hard.json");
        let questions: Questions = serde_json::from_str(data).unwrap();
        questions
    }
}
