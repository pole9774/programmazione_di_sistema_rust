use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    text: String,
}

fn capitalize_word(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn capitalize(s: &str) -> String {
    let mut result = String::new();
    for word in s.split(' ') {
        result.push_str(&capitalize_word(word));
        result.push(' ');
    }
    result.pop();
    result
}

fn main() {
    let args = Args::parse();
    
    println!("{}", capitalize(&args.text));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::capitalize(&"prova provetta"), "Prova Provetta");
    }
}

// cargo run "hello world"
