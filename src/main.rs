use clap::{Parser, Subcommand};

mod game;
mod lang;
mod tui;

use crate::game::get_random_words;
use crate::lang::get_words;
use crate::tui::run_typing_test;

#[derive(Parser, Debug)]
#[command(name = "keyzen", version, about = "Typing test in CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Start {
        #[arg(short, long, default_value_t = 60)]
        duration: u32,

        #[arg(short, long, default_value = "rust")]
        lang: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start { duration, lang } => {
            println!("Starting typing test for {duration} sec in {lang}");

            let words = get_words(lang);
            let random_words = get_random_words(&words, 10);

            run_typing_test(random_words, *duration).unwrap();
        }
    }
}
