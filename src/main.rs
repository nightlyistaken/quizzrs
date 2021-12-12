use std::str::FromStr;
use std::io::Write;
use std::fs::File;
mod difficulties;
mod quiz;
use quiz::Quiz;
use quiz::Difficulty;
use std::env;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Quizzr {
    tts: String,
    difficulty: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Quizzr {
        tts: env::var("TTS")?,
        difficulty: env::var("DIFFICULTY")?,
    };
    if args.tts.is_empty() {
        return Err("TTS is not set".into());
    }
    if args.difficulty.is_empty() {
        return Err("DIFFICULTY is not set".into());
    }
    let quiz = Quiz::new(Difficulty::from_str(&args.difficulty)?);
    println!("{:?}", args);
    
    // this is where we will store results of the quiz
    let mut results: [bool; 4] = [false; 4];
    for val in results.iter_mut() {
        let answer = quiz.ask(args.clone());
        // magic happens here
        *val = answer.correct;
    }
    // print results to file
    let mut file = File::create("results.txt")?;
    let mut string = String::new();
    for val in results.iter() {
        string.push_str(val.to_string().as_str());
        string.push('\n');
    }
    file.write_all(string.as_bytes())?;

    // print Quizzr struct to file
    let mut file = File::create("quizzr.json")?;
    let string = serde_json::to_string(&args)?;
    file.write_all(string.as_bytes())?;
    
    
    Ok(())
}

#[test]
fn quiz_test() {
    let quiz = Quiz::new(Difficulty::Medium);
    let args = Quizzr {
        tts: "false".to_string(),
        difficulty: "easy".to_string(),
    };
    let answer = quiz.ask(args);
    assert!(answer.correct);
}