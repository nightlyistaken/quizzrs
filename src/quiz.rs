use crate::Identifier;
use rand::seq::SliceRandom;
#[derive(Clone, Copy)]
pub struct Quiz;

impl Quiz {
    fn readline() -> String {
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();
        user_input
    }

    pub fn ask(tts: &str) -> bool {
        let client = tts_rust::GTTSClient {
            volume: 1.0,
            language: tts_rust::languages::Languages::English,
        };

        let data = &Identifier::identify_json();
        let question = data.questions.choose(&mut rand::thread_rng()).unwrap();
        println!("{}", &question.question);
        if tts == "true" {
            client.speak(&question.question)
        }
        for index in 0..4 {
            println!("{}. {:?}", index, &question.answers[index]);
        }
        let answer: i32 = Quiz::readline().trim().parse().unwrap();
        answer == question.correct_index
    }
}