use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // When placed here the entire spawned thread will run
    // handle.join().unwrap()

    let v = vec![1,2,3];

    let handle_2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle_2.join().unwrap()

}
