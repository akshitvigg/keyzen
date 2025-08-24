use clap::{Parser, Subcommand};
mod game;
mod lang;
mod tui;

use crate::game::get_random_words;
use crate::lang::{get_available_languages, get_words};
use crate::tui::run_typing_test;

#[derive(Parser, Debug)]
#[command(
    name = "keyzen", 
    version, 
    about = "A terminal-based typing speed test application.\nSupports multiple programming languages like Python, JavaScript, Go, and more.",
    after_help = "Use \"keyzen [command] --help\" for more information about a command."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(
        after_help = "Examples:\n  keyzen start --lang python\n  keyzen start --duration 30 --lang javascript\n  keyzen start --list-langs"
    )]
    Start {
        #[arg(short, long, default_value_t = 30)]
        duration: u32,
        
        #[arg(short, long, default_value = "english")]
        lang: String,
        
        #[arg(long)]
        list_langs: bool,
    },
    
    Languages,
    
    Version,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Start { duration, lang, list_langs } => {
            if *list_langs {
                println!("Available languages:");
                for lang in get_available_languages() {
                    let word_count = get_words(&lang).len();
                    println!("  {} ({} words)", lang, word_count);
                }
                return;
            }

            let available_langs = get_available_languages();
            if !available_langs.contains(lang) {
                eprintln!("Language '{}' not found.", lang);
                eprintln!("Available: {}", available_langs.join(", "));
                return;
            }

            loop {
                let word_pool = get_words(lang);
                let estimated_words_needed = (*duration as f64 * 60.0 / 60.0) as usize; // 60 WPM estimate
                let random_words = get_random_words(&word_pool, estimated_words_needed.max(50));
                
                match run_typing_test(random_words, *duration, lang) {
                    Ok(()) => {
                    },
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        break;
                    }
                }
            }
        }
        Commands::Languages => {
            println!("Available languages:");
            for lang in get_available_languages() {
                let word_count = get_words(&lang).len();
                println!("  {} ({} words)", lang, word_count);
            }
        }
        Commands::Version => {
            println!("keyzen version {}", env!("CARGO_PKG_VERSION"));
        }
    }
}