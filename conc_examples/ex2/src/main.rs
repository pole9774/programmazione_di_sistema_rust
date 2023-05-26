use std::time::Duration;
use std::thread;
use std::thread::sleep;
use std::sync::Arc;

mod cdl {
    use std::sync::{Condvar, Mutex};
    
    pub struct Cdl {
        m: Mutex<usize>,
        cv: Condvar
    }

    impl Cdl {
        pub fn new(count: usize) -> Self {
            Cdl {
                m: Mutex::new(count),
                cv: Condvar::new()
            }
        }

        pub fn count_down(&self) {
            let mut s = self.m.lock().unwrap();
            if *s == 0 {
                panic!("Count is already 0");
            }
            *s -= 1;
            if *s == 0 {
                self.cv.notify_all();
            }
        }

        pub fn wait(&self) {
            let mut s = self.m.lock().unwrap();
            while *s > 0 {
                s = self.cv.wait(s).unwrap();
            }
        }
    }
}

fn main() {
    let c = Arc::new(cdl::Cdl::new(2));
    let cdl1 = Arc::clone(&c);
    let cdl2 = Arc::clone(&c);

    let t1 = thread::spawn(move || {
        sleep(Duration::from_secs(2));
        cdl1.count_down();
        sleep(Duration::from_secs(2));
        println!("Thread1 done");
    });

    let t2 = thread::spawn(move || {
        sleep(Duration::from_secs(3));
        cdl2.count_down();
        sleep(Duration::from_secs(6));
        println!("Thread1 done");
    });

    c.wait();
    println!("Done");
    t1.join().unwrap();
    t2.join().unwrap();
}
