use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // wrap the Mutex to be able to share ownership across multiple threads
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];

    for i in 0..2 {
        let data = Arc::clone(&data); // clone the reference
        let handle = thread::spawn(move || {
            let mut data = data.lock().unwrap(); // lock for a mutable reference
            for j in 0..data.len() {
                data[j] += i;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
