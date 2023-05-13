use minesweeper::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rows: usize,
    #[arg(short, long)]
    pcols: usize,
    #[arg(short, long)]
    minefield: String,
    #[arg(short, long, default_value_t = 3)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    let s = args.minefield.chars().collect::<Vec<_>>().chunks(args.pcols).map(|c| c.iter().collect::<String>()).collect::<Vec<_>>();

    let mut s1 = Vec::<&str>::with_capacity(s.len());

    for tmp in &s {
        s1.push(tmp);
    }

    println!("{:?}", annotate(&s1));
}

// cargo run -- --rows=3 --pcols=3 --minefield="*  *  *  "