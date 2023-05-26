use std::thread;
use std::thread::JoinHandle;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let v: Vec<JoinHandle<i32>> = (1..=4).map(|tid| {
        thread::spawn(move || {
            (1..10).for_each(|i| {
                println!("Thread #{}, iterazione #{}", tid, i);
                sleep(Duration::from_secs(1));
            });
            if tid == 3 {panic!("Computazione fallita");}
            println!("Thread #{} fatto", tid);
            tid
        })
    }).collect();

    println!("Thread principale");
    sleep(Duration::from_secs(4));
    println!("Attendo terminazione thread secondari");
    v.into_iter().for_each(|h| {
        let res = h.join();
        match res {
            Ok(val) => println!("Res = {}", val),
            Err(e) => println!("Error {:?}", e),
        }
    });
    println!("Thread terminato");
}
