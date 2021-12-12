pub struct MediumDifficulty;
use crate::difficulties::typers::Questions;


impl MediumDifficulty {
    pub fn get_questions() -> Questions {
        let data = include_str!("../data/content.json");
        let questions: Questions = serde_json::from_str(data).unwrap();
        questions
    }
}
