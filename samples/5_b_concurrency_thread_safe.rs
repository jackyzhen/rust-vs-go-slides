use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // multiple producer, single receiver
    let (c_tx, c_rx) = channel();
    let mut ref_c = 0;
    let mut xs = Vec::new(); 

    for i in 0..100 {
        thread::spawn(|| {
            ref_c = ref_c + 1;
            xs.push(i);
            c_tx.send(true).unwrap();
        });
    }

    for _i in 0..100 {
        c_rx.recv().unwrap();
    }

    println!("{:?}", ref_c);
    println!("{:?}", xs.len());
}
