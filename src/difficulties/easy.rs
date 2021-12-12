pub struct EasyDifficulty;
use crate::difficulties::typers::Questions;


impl EasyDifficulty {
    pub fn get_questions() -> Questions {
        let data = include_str!("../data/content.json");
        let questions: Questions = serde_json::from_str(data).unwrap();
        questions
    }
}
