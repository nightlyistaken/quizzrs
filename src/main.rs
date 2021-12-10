use quicli::prelude::*;
use structopt::StructOpt;
mod identifier;
mod quiz;

use identifier::Identifier;

#[derive(Debug, StructOpt)]
struct MusicCLI {
    #[structopt(long = "tts", default_value = "true")]
    tts: String,
}

fn main() -> CliResult {
    let args = MusicCLI::from_args();

    println!(
        "Starting quizzr with these: tts = {} and difficulty = normal",
        args.tts
    );
    let q1 = quiz::Quiz::ask(&args.tts);
    let q2 = quiz::Quiz::ask(&args.tts);
    let q3 = quiz::Quiz::ask(&args.tts);
    let q4 = quiz::Quiz::ask(&args.tts);
    let q5 = quiz::Quiz::ask(&args.tts);

    if q1 == q2 && q2 == q3 && q3 == q4 && q4 == q5 {
        println!("You win!")
    } else {
        println!("You lose!")
    }
    Ok(())
}
