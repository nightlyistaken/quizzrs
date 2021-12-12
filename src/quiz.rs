use std::str::FromStr;
use crate::Quizzr;
use rand::seq::SliceRandom;
use crate::difficulties::easy::EasyDifficulty;
use crate::difficulties::medium::MediumDifficulty;
use crate::difficulties::hard::HardDifficulty;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}
impl FromStr for Difficulty {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "easy" => Ok(Difficulty::Easy),
            "medium" => Ok(Difficulty::Medium),
            "hard" => Ok(Difficulty::Hard),
            _ => Err(String::from("Invalid difficulty. use easy, medium or hard")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct QuizResult {
    pub correct: bool,
    pub incorrect: bool,
}


pub struct Quiz {
    pub difficulty: Difficulty,
}

impl Quiz {
    pub fn new(diff: Difficulty) -> Quiz {
        Quiz { difficulty: diff }
    }
    fn readline() -> String {
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();
        user_input
    }
    pub fn ask(&self, quizzr_client: Quizzr) -> QuizResult {
        let client = tts_rust::GTTSClient {
            volume: 1.0,
            language: tts_rust::languages::Languages::English,
        };

        let easy_data = EasyDifficulty::get_questions();
        let medium_data = MediumDifficulty::get_questions();
        let hard_data = HardDifficulty::get_questions();
        
        let easy_question = easy_data.questions.choose(&mut rand::thread_rng()).unwrap();
        let medium_question = medium_data.questions.choose(&mut rand::thread_rng()).unwrap();
        let hard_question = hard_data.questions.choose(&mut rand::thread_rng()).unwrap();
        let question = match self.difficulty {
            Difficulty::Easy => easy_question,
            Difficulty::Medium => medium_question,
            Difficulty::Hard => hard_question,
        };
        
        println!("{}", &question.question);
        if quizzr_client.tts == "true" {
            client.speak(&question.question)
        }
        for index in 0..4 {
            println!("{}. {:?}", index, &question.answers[index]);
        }
        let answer: i32 = Quiz::readline().trim().parse().unwrap();
        QuizResult {
            correct: if answer == question.correct_index { true } else { false },
            incorrect: if answer != question.correct_index { true } else { false },
        }
    }
}