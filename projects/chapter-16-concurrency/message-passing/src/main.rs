use std::{sync::mpsc, time::Duration};
use std::thread;


fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
        // let val = String::from("hi");
        // tx.send(val).unwrap();
        // the below won't work, uncomment to find out why.
        // println!("val is {}", val);
    });

    thread::spawn(move || {

        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
        // let val = String::from("hi");
        // tx.send(val).unwrap();
        // the below won't work, uncomment to find out why.
        // println!("val is {}", val);
    });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    for received in rx {
        println!("Got: {}", received);
    }
}

// Simple single producer single consumer

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {

//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1))
//         }
//         // let val = String::from("hi");
//         // tx.send(val).unwrap();
//         // the below won't work, uncomment to find out why.
//         // println!("val is {}", val);
//     });

//     // let received = rx.recv().unwrap();
//     // println!("Got: {}", received);

//     for received in rx {
//         println!("Got: {}", received);
//     }
// }
