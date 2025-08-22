use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about , long_about=None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u32,
}

fn main() {
    let arg = Args::parse();

    for _ in 0..arg.count {
        println!("Hello {}", arg.name);
    }
}
