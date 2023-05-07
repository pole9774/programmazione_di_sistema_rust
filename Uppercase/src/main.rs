use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

// makes the first letter of every word in the string uppercase
fn capitalize(s: &str) -> String {
    let mut res = "".to_string();
    let mut last_space = true;

    for c in s.chars() {
        if last_space && c.is_ascii_alphabetic() {
            res.push(c.to_ascii_uppercase());
        } else {
            res.push(c);
        }
        last_space = c.is_ascii_whitespace();
    }

    return res;
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("{}", capitalize(&args.name));
    }
}


// cargo run -- --name "hello world"
