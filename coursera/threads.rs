use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
    }

    handle.join().unwrap();
}
