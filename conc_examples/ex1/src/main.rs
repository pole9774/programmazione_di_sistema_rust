use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::ops::Add;

fn main() {
    let state = Arc::new(Mutex::new(Vec::<String>::new()));
    let h: Vec<JoinHandle<()>> = (1..=2).map(|tid| {
        let state = state.clone();
        thread::spawn(move || {
            for i in 1..500 {
                state.lock().unwrap().push(i.to_string().add("-").add(tid.to_string().as_str()));
            }
        })
    }).collect();
    h.into_iter().for_each(|h| { h.join().unwrap(); });
    println!("=============================");
    state.lock().unwrap().iter().for_each(|s| {println!("{}", s); });
}
