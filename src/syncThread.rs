use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::process;

fn main() {
    let counter = Arc::new((Mutex::new(1), Condvar::new()));
    let word1 = Arc::new("ED".to_string());
    let word2 = Arc::new("CD".to_string());
    let all_threads_done = Arc::new(Mutex::new(false));

    let handle1 = {
        let counter = Arc::clone(&counter);
        let word1 = Arc::clone(&word1);
        let word2 = Arc::clone(&word2);
        let all_threads_done = Arc::clone(&all_threads_done);
        thread::spawn(move || {
            for _ in 0..5 {
                let (mutex, condvar) = &*counter;
                let mut counter = mutex.lock().unwrap();

                println!("{} {} {}", *counter, word1, word2);

                *counter += 1;
                condvar.notify_one();
                counter = condvar.wait_while(counter, |&mut c| c % 2 == 0).unwrap();
            }
            *all_threads_done.lock().unwrap() = true;
        })
    };

    let handle2 = {
        let counter = Arc::clone(&counter);
        let word1 = Arc::clone(&word1);
        let word2 = Arc::clone(&word2);
        let all_threads_done = Arc::clone(&all_threads_done);
        thread::spawn(move || {
            for _ in 0..5 {
                let (mutex, condvar) = &*counter;
                let mut counter = mutex.lock().unwrap();

                *counter += 1;
                condvar.notify_one();
                counter = condvar.wait_while(counter, |&mut c| c % 2 != 0).unwrap();

                println!("{} {} {}", *counter, word1, word2);
            }
            *all_threads_done.lock().unwrap() = true;
        })
    };

    handle1.join().unwrap();
    handle2.join().unwrap();
}
