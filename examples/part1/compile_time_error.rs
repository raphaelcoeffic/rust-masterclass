use std::thread;

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    let mut handles = vec![];

    for i in 0..2 {
        let handle = thread::spawn(move || {
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
