mod completions;

use std::io::{self, Write};

use clap::Parser;
use completions::OpenAIClient;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start app as a conversation
    #[arg(short, long)]
    chat: bool,
}

fn main() {
    let args = Args::parse();
    let mut ai = OpenAIClient {
        client: OpenAIClient::new(),
        conversation: vec![],
    };

    println!(
        "--------------------------------WELCOME TO AI TERMINAL--------------------------------"
    );
    if args.chat {
        loop {
            print!("Question: ");
            let _ = io::stdout().flush();
            let mut question = String::new();
            io::stdin()
                .read_line(&mut question)
                .expect("Failed to read line");

            let question = question.trim();

            if ["q", "quit", "exit"].contains(&question.to_lowercase().as_str()) {
                println!("Bye bye");
                break;
            }

            print!("AI: ");
            ai.chat(&question).unwrap();
            println!("\n------------------------------------------------");
        }
    }
}
