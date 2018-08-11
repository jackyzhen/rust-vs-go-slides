use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let (c_tx, c_rx) = channel();
    let ref_c = 0;
    let xs = Vec::new(); 
    
    // Wrap data in a mutex lock in an atomic ref counter...
    let c_arc = Arc::new(Mutex::new(ref_c));
    let xs_arc = Arc::new(Mutex::new(xs));

    for i in 0..100 {
        // cloning ARC bumps counter up and gives a thread safe passable ref
        let c_arc = c_arc.clone();
        let xs_arc = xs_arc.clone();
        let c_tx = c_tx.clone();
        thread::spawn(move || {
            
            // safely mutate shared data after locking
            let mut ref_c = c_arc.lock().unwrap();
            *ref_c = *ref_c + 1;
            
            let mut xs = xs_arc.lock().unwrap();
            xs.push(i);
            
            c_tx.send(true).unwrap();
            
            
        }); // locks are dropped/unlocked at end of ownership scope
    }

    for _ in 0..100 {
        c_rx.recv().unwrap();
    }
    
    let ref_c = c_arc.lock().unwrap();
    let xs = xs_arc.lock().unwrap();
    println!("{:?}", *ref_c);
    println!("{:?}", xs.len());
}
