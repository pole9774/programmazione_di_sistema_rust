use luhn::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

// checks if a string is made of 4 groups of 4 digits, with a space between groups
fn is_credit_card(code: &str) -> bool {
    let mut i = 0;

    if code.len() != 19 {
        return false;
    }

    for c in code.chars() {
        if i == 4 || i == 9 || i == 14 {
            if !(c.is_whitespace()) {
                return false;
            }
        } else {
            if !(c.is_digit(10)) {
                return false;
            }
        }
        i = i + 1;
    }

    return true;
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        if !(is_credit_card(&args.name)) {
            println!("Invalid format: NOT 4 groups of 4 digits!");
        } else {
            println!("Valid format: Luhn's algorithm -> {}", is_valid(&args.name));
        }
    }
}