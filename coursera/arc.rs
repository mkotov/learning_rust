use std::thread;
use std::sync::Mutex;
use std::sync::Arc;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let counter1 = Arc::clone(&counter);
    let counter2 = Arc::clone(&counter);

    let handle1 = thread::spawn(move || {
        let mut num = counter1.lock().unwrap();
        *num += 1;
    });

    let handle2 = thread::spawn(move || {
        let mut num = counter2.lock().unwrap();
        *num += 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("Counter: {:?}", counter.lock().unwrap());
}
