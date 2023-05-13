use clock::Clock;

fn main() {
    let mut c = Clock::new(0, 50);
    println!("{}", c);
    c = c.add_minutes(-30);
    println!("{}", c);

    let clock = Clock::new(2, 20).add_minutes(-3000);
    println!("{}", clock);

    if c == clock {
        println!("Uguali");
    } else {
        println!("Diversi");
    }
}